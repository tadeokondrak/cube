use anyhow::{Context as _, Result};
use calamine::{open_workbook_auto, DataType, Reader};
use cube::{CornerSticker, Cube, EdgeSticker, WingSticker};
use cube_bld::{memo, Orientation, Permutation, Pieces};
use cube_notation::{canonicalize, format_moves, parse_alg, ParseMode, Tree};
use indicatif::{MultiProgress, ParallelProgressIterator, ProgressBar, ProgressStyle};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use reqwest::blocking::Client;
use rquickjs::{CatchResultExt, Context, Runtime, Undefined};
use std::{
    collections::BTreeMap,
    fmt::Display,
    fs,
    io::{self, BufRead, BufReader, Write},
    path::PathBuf,
    sync::Arc,
    time::Duration,
};
use struson::reader::JsonReader;

fn letter(x: u8) -> char {
    if x < 24 {
        (x + b'A') as char
    } else {
        assert!(x < 48);
        (x - 24 + b'a') as char
    }
}

fn get_url(path: &str, url: &str) -> anyhow::Result<PathBuf> {
    fs::create_dir_all("cache")?;
    let cache_path = PathBuf::from(format!("cache/{path}"));
    if cache_path.try_exists()? {
        return Ok(cache_path);
    }
    eprintln!("Downloading {path} from {url}...");
    let response = Client::builder()
        .timeout(Duration::from_secs(120))
        .build()?
        .get(url)
        .send()?;
    let mut file = fs::File::create(&cache_path)?;
    io::copy(&mut response.bytes().unwrap().as_ref(), &mut file)?;
    Ok(cache_path)
}

fn main() -> Result<()> {
    let mut sheets_csv = csv::Reader::from_path("data/google.csv")?;
    let google_sheets = sheets_csv.deserialize::<(String, String)>().map(|res| {
        res.map_err(|e| anyhow::Error::from(e))
            .and_then(|(name, key)| {
                let url =
                    format!("https://docs.google.com/spreadsheets/d/{key}/export?format=xlsx");
                Ok((url, key, name))
            })
    });

    let mut excel_csv = csv::Reader::from_path("data/excel.csv")?;
    let excel_sheets = excel_csv
        .deserialize::<(String, String, String)>()
        .map(|res| {
            res.map_err(|e| anyhow::Error::from(e))
                .and_then(|(name, key, url)| Ok((url, key, name)))
        });

    let mut blddb_csv = csv::Reader::from_path("data/blddb.csv")?;
    let blddb_files = blddb_csv
        .deserialize::<(String, String)>()
        .map(|res| {
            res.map_err(|e| anyhow::Error::from(e))
                .and_then(|(key, url)| {
                    let path = format!("blddb_{key}.json");
                    let loc = get_url(&path, &url)?;
                    Ok((url, loc, key))
                })
        })
        .collect::<anyhow::Result<Vec<(String, PathBuf, String)>>>()?;

    let mut simple_csv = csv::Reader::from_path("data/simple.csv")?;
    let simple_files = simple_csv
        .deserialize::<(String, String, String)>()
        .map(|res| {
            res.map_err(|e| anyhow::Error::from(e))
                .and_then(|(name, key, url)| {
                    let path = format!("{key}.txt");
                    let loc = get_url(&path, &url)?;
                    Ok((url, loc, name))
                })
        })
        .collect::<anyhow::Result<Vec<(String, PathBuf, String)>>>()?;

    let all_sheets = google_sheets
        .chain(excel_sheets)
        .map(|result| {
            result.and_then(|(url, key, name)| {
                let path = format!("{key}.xlsx");
                let loc = get_url(&path, &url)?;
                Ok((url, loc, name))
            })
        })
        .collect::<anyhow::Result<Vec<(String, PathBuf, String)>>>()?;

    let progress = MultiProgress::new();

    let count = (all_sheets.len() + simple_files.len() + blddb_files.len()) as u64;

    let mut all_algs = make_sheets_iter(all_sheets, progress.clone())
        .chain(make_blddb_iter(blddb_files, progress.clone()))
        .chain(make_text_iter(simple_files, progress.clone()))
        .progress_with(
            progress.add(
                ProgressBar::new(count)
                    .with_style(ProgressStyle::with_template("{bar} {pos}/{len} {msg}").unwrap())
                    .with_message("All"),
            ),
        )
        .flat_map_iter(|result| match result {
            Ok(vec) => vec.into_iter().map(Ok).collect(),
            Err(e) => vec![Err(e)],
        })
        .collect::<Result<Vec<(Case, String, Arc<Source>)>>>()?;

    all_algs.sort();
    all_algs.dedup();

    let mut map: BTreeMap<
        String,
        BTreeMap<Case, BTreeMap<String, BTreeMap<String, Vec<Arc<Source>>>>>,
    > = BTreeMap::new();
    for (case, alg, source) in all_algs {
        let tree = parse_alg(case.n(), ParseMode::Wca, &alg).unwrap();
        let canonical = format_moves(&canonicalize(&tree.to_moves()));
        let fancy = tree.to_string();
        map.entry(case.category_name().to_owned())
            .or_default()
            .entry(case)
            .or_default()
            .entry(canonical)
            .or_default()
            .entry(fancy)
            .or_default()
            .push(source);
    }

    for (category_name, cases) in map {
        std::fs::create_dir_all("out")?;
        let mut writer = std::fs::File::create(format!("out/{}.json", category_name)).unwrap();
        let mut out = String::new();
        {
            let mut source_cache: Vec<Arc<Source>> = Vec::new();
            let mut obj = write_json::object(&mut out);
            let mut obj = obj.object("cases");
            for (case, canonicals) in cases {
                let mut obj = obj.object(&case.to_string());
                {
                    for (canonical, variants) in canonicals {
                        let mut obj = obj.object(&format!("{canonical}"));
                        for (variant, mut sources) in variants {
                            let mut arr = obj.array(&format!("{variant}"));
                            sources.sort();
                            sources.dedup();
                            for source in sources {
                                let i = source_cache
                                    .iter()
                                    .position(|x| Arc::ptr_eq(&source, x))
                                    .unwrap_or_else(|| {
                                        let i = source_cache.len();
                                        source_cache.push(source.clone());
                                        i
                                    });
                                arr.number(i as f64);
                            }
                        }
                    }
                }
            }
            let mut arr = obj.array("sources");
            for source in source_cache {
                match &*source {
                    Source::Custom { url, name } => {
                        arr.object()
                            .object("custom")
                            .string("url", url)
                            .string("name", name);
                    }
                    Source::Spreadsheet {
                        author,
                        url,
                        sheet_name,
                    } => {
                        arr.object()
                            .object("spreadsheet")
                            .string("url", url)
                            .string("user", author)
                            .string("sheet", sheet_name);
                    }
                }
            }
        }
        out.push('\n');
        writer.write_all(out.as_bytes()).unwrap();
    }

    Ok(())
}

fn make_sheets_iter(
    sheets: Vec<(String, PathBuf, String)>,
    progress: MultiProgress,
) -> impl ParallelIterator<Item = Result<Vec<(Case, String, Arc<Source>)>>> {
    let len = sheets.len() as u64;
    sheets
        .into_par_iter()
        .map({
            let progress = progress.clone();
            move |(url, loc, name)| {
                let is_excel_file = loc.extension().is_some_and(|x| x == "xls" || x == "xlsx");
                if !is_excel_file {
                    return Ok(Vec::new());
                }

                let mut algs = Vec::new();

                let mut book = open_workbook_auto(&loc).context(url.clone())?;
                for (sheet_name, sheet) in book.worksheets() {
                    let source = Arc::new(Source::Spreadsheet {
                        author: name.to_owned(),
                        url: url.clone(),
                        sheet_name: sheet_name.clone(),
                    });
                    let heuristics = Heuristics::compute(&source);
                    if heuristics.should_be_ignored {
                        continue;
                    }
                    let cells = sheet.cells();
                    let progress = progress.add(
                        ProgressBar::new(cells.len() as u64)
                            .with_style(
                                ProgressStyle::with_template("{bar} {pos}/{len} {msg}").unwrap(),
                            )
                            .with_message(format!("{name} {sheet_name}")),
                    );
                    for (_x, _y, content) in cells {
                        if let DataType::String(text) = content {
                            for line in text.lines() {
                                if line.is_empty() || !line.contains(' ') {
                                    continue;
                                }

                                collect(line, &heuristics, |case, alg| {
                                    algs.push((case, alg.to_string(), source.clone()));
                                });
                            }
                        }
                        progress.inc(1);
                    }
                }

                Ok(algs)
            }
        })
        .progress_with(
            progress.add(
                ProgressBar::new(len)
                    .with_style(ProgressStyle::with_template("{bar} {pos}/{len} {msg}").unwrap())
                    .with_message(format!("Spreadsheets")),
            ),
        )
}

fn make_blddb_iter(
    files: Vec<(String, PathBuf, String)>,
    progress: MultiProgress,
) -> impl ParallelIterator<Item = Result<Vec<(Case, String, Arc<Source>)>>> {
    let len = files.len() as u64;
    files
        .into_par_iter()
        .map({
            let progress = progress.clone();
            move |(_url, loc, name)| {
                let file = std::fs::File::open(&loc)?;
                let mut reader = struson::reader::JsonStreamReader::new(file);
                reader.begin_object()?;

                let source = Arc::new(Source::Custom {
                    url: "https://blddb.net/nightmare.html".to_owned(),
                    name: "BLDDB".to_owned(),
                });

                let mut lines = Vec::new();
                while reader.has_next()? {
                    _ = reader.next_name()?;
                    lines.push(reader.next_string()?);
                }

                let len = lines.len() as u64;
                Ok(lines
                    .into_par_iter()
                    .progress_with(
                        progress.add(
                            ProgressBar::new(len)
                                .with_style(
                                    ProgressStyle::with_template("{bar} {pos}/{len} {msg}")
                                        .unwrap(),
                                )
                                .with_message(format!("BLDDB: {name}")),
                        ),
                    )
                    .map_init(
                        || {
                            let runtime = Runtime::new().unwrap();
                            let context = Context::full(&runtime).unwrap();
                            context
                                .with(|ctx| {
                                    Undefined = ctx
                                        .eval(include_bytes!("commutator.js").as_slice())
                                        .catch(&ctx)
                                        .map_err(|e| anyhow::format_err!("{e:?}"))?;
                                    Ok::<_, anyhow::Error>(())
                                })
                                .unwrap();
                            (runtime, context)
                        },
                        |(_runtime, context), alg| {
                            let converted_alg = context
                                .with(|ctx| {
                                    Ok::<String, rquickjs::Error>(
                                        ctx.globals()
                                            .get::<_, rquickjs::Function>("commutator")?
                                            .call((&alg,))?,
                                    )
                                })
                                .unwrap();

                            let alg = if converted_alg == "Not found." {
                                alg
                            } else {
                                converted_alg
                            };

                            let mut algs = Vec::new();
                            collect(&alg, &Heuristics::compute(&source), |case, alg| {
                                algs.push((case, alg.to_string(), source.clone()));
                            });
                            algs
                        },
                    )
                    .flatten_iter()
                    .collect::<Vec<_>>())
            }
        })
        .progress_with(
            progress.add(
                ProgressBar::new(len)
                    .with_style(ProgressStyle::with_template("{bar} {pos}/{len} {msg}").unwrap())
                    .with_message(format!("BLDDB")),
            ),
        )
}

fn make_text_iter(
    files: Vec<(String, PathBuf, String)>,
    progress: MultiProgress,
) -> impl ParallelIterator<Item = Result<Vec<(Case, String, Arc<Source>)>>> {
    let len = files.len() as u64;
    files
        .into_par_iter()
        .map(|(url, loc, name)| {
            let source = Arc::new(Source::Custom { url, name });
            let file = BufReader::new(std::fs::File::open(&loc)?);
            let mut algs = Vec::new();
            for line in file.lines() {
                let line = line?;
                collect(&line, &Heuristics::compute(&source), |case, alg| {
                    algs.push((case, alg.to_string(), source.clone()));
                });
            }
            Ok(algs)
        })
        .progress_with(
            progress.add(
                ProgressBar::new(len)
                    .with_style(ProgressStyle::with_template("{bar} {pos}/{len} {msg}").unwrap())
                    .with_message(format!("Text files")),
            ),
        )
}

#[derive(Hash, Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord)]
enum Case {
    Corner3Cycle([CornerSticker; 3]),
    Edge3Cycle([EdgeSticker; 3]),
    Wing3Cycle([EdgeSticker; 3]),
    Midge3Cycle([EdgeSticker; 3]),
    XCenter3Cycle([CornerSticker; 3]),
    TCenter3Cycle([EdgeSticker; 3]),
    LeftOblique3Cycle([EdgeSticker; 3]),
    RightOblique3Cycle([EdgeSticker; 3]),
    Corner2SwapEdge2Swap([CornerSticker; 2], [EdgeSticker; 2]),
    Wing2Swap([WingSticker; 2]),
    // Corner2Twist([CornerSticker; 2]),
    // Corner3Twist([CornerSticker; 3]),
    // Edge2Flip([EdgeSticker; 2]),
    // Edge2SwapEdge2Swap([EdgeSticker; 2], [EdgeSticker; 2]),
    // Corner2SwapCorner2Swap([CornerSticker; 2], [CornerSticker; 2]),
}

impl Case {
    fn category_name(self) -> &'static str {
        match self {
            Case::Corner3Cycle(_) => "Corner3Cycle",
            Case::Edge3Cycle(_) => "Edge3Cycle",
            Case::Wing3Cycle(_) => "Wing3Cycle",
            Case::Midge3Cycle(_) => "Midge3Cycle",
            Case::XCenter3Cycle(_) => "XCenter3Cycle",
            Case::TCenter3Cycle(_) => "TCenter3Cycle",
            Case::LeftOblique3Cycle(_) => "LeftOblique3Cycle",
            Case::RightOblique3Cycle(_) => "RightOblique3Cycle",
            Case::Corner2SwapEdge2Swap(_, _) => "Corner2SwapEdge2Swap",
            Case::Wing2Swap(_) => "Wing2Swap",
        }
    }

    fn n(self) -> u16 {
        match self {
            Case::Corner2SwapEdge2Swap(_, _) | Case::Corner3Cycle(_) | Case::Edge3Cycle(_) => 3,
            Case::Wing2Swap(_) | Case::Wing3Cycle(_) | Case::XCenter3Cycle(_) => 4,
            Case::Midge3Cycle(_) | Case::TCenter3Cycle(_) => 5,
            Case::LeftOblique3Cycle(_) | Case::RightOblique3Cycle(_) => 6,
        }
    }
}

impl Display for Case {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Case::Wing2Swap([a, b]) => {
                write!(f, "{}{}", letter(a.index() as u8), letter(b.index() as u8))
            }
            Case::Corner3Cycle([a, b, c]) | Case::XCenter3Cycle([a, b, c]) => {
                write!(
                    f,
                    "{}{}{}",
                    letter(a.index() as u8),
                    letter(b.index() as u8),
                    letter(c.index() as u8)
                )
            }
            Case::Edge3Cycle([a, b, c])
            | Case::Midge3Cycle([a, b, c])
            | Case::TCenter3Cycle([a, b, c])
            | Case::LeftOblique3Cycle([a, b, c])
            | Case::RightOblique3Cycle([a, b, c]) => {
                write!(
                    f,
                    "{}{}{}",
                    letter(a.index() as u8),
                    letter(b.index() as u8),
                    letter(c.index() as u8)
                )
            }
            Case::Wing3Cycle([a, b, c]) => {
                write!(
                    f,
                    "{}{}{}",
                    letter(a.index() as u8),
                    letter(b.index() as u8),
                    letter(c.index() as u8)
                )
            }
            Case::Corner2SwapEdge2Swap([a, b], [c, d]) => {
                write!(
                    f,
                    "{}{}{}{}",
                    letter(a.index() as u8),
                    letter(b.index() as u8),
                    letter(c.index() as u8),
                    letter(d.index() as u8)
                )
            }
        }
    }
}

#[derive(Debug)]
struct Heuristics {
    should_be_ignored: bool,
    is_possibly_3x3: bool,
    is_possibly_4x4: bool,
    is_possibly_5x5: bool,
    is_possibly_6x6: bool,
    is_possibly_midges: bool,
    is_possibly_wings: bool,
    is_probably_tcenters: bool,
    is_possibly_xcenters: bool,
    is_probably_midges: bool,
}

impl Heuristics {
    fn compute(source: &Source) -> Heuristics {
        let (sheet_name, author) = match source {
            Source::Custom { .. } => ("", ""),
            Source::Spreadsheet {
                author,
                url: _,
                sheet_name,
            } => (sheet_name.as_str(), author.as_str()),
        };

        let is_definitely_edges = sheet_name.contains("Edges") || sheet_name.contains("edges");
        let is_definitely_midges = sheet_name.contains("Midges")
            || sheet_name.contains("midges")
            || sheet_name == "M" && author != "Bertie Longden"
            || sheet_name == "m";
        let is_definitely_wings = sheet_name.contains("Wings")
            || sheet_name.contains("wings")
            || sheet_name.contains("UFr");
        let is_definitely_tcenters = sheet_name.contains("T-centers")
            || sheet_name.contains("+-centers")
            || sheet_name.contains("+-centres")
            || sheet_name == "t";
        let is_definitely_xcenters =
            sheet_name.contains("X-centers") || sheet_name.contains("x Center Comms");

        let is_possibly_3x3 = !sheet_name.contains("Center")
            && !sheet_name.contains("center")
            && !sheet_name.contains('+')
            && !is_definitely_wings
            && !is_definitely_tcenters
            && !is_definitely_midges;

        let is_definitely_6x6 = sheet_name.contains("6BLD");

        let is_possibly_4x4 = !is_definitely_6x6;
        let is_possibly_5x5 = !is_definitely_6x6;
        let is_possibly_6x6 = true;

        let is_possibly_midges = !is_definitely_tcenters && !is_definitely_edges;

        let is_possibly_wings = !is_definitely_xcenters && !is_definitely_midges;

        let is_probably_tcenters =
            sheet_name.contains('+') || sheet_name.contains("T-centers") || sheet_name == "t";

        let is_possibly_xcenters = !sheet_name.contains("6BLD") && !is_definitely_wings;

        let is_probably_midges = !is_definitely_tcenters
            && is_possibly_midges
            && (is_definitely_midges
                || (sheet_name.starts_with('m')
                    || sheet_name.starts_with('M')
                    || (author == "James Molloy" && sheet_name == "M - UF")));

        Heuristics {
            should_be_ignored: sheet_name.starts_with("Scrape")
                || sheet_name.starts_with("Input")
                || sheet_name.contains("Ignore")
                || sheet_name.contains("Old"),
            is_possibly_3x3,
            is_possibly_4x4,
            is_possibly_5x5,
            is_possibly_6x6,
            is_possibly_midges,
            is_possibly_wings,
            is_probably_tcenters,
            is_possibly_xcenters,
            is_probably_midges,
        }
    }
}

fn collect(
    line: &str,
    &Heuristics {
        should_be_ignored: _,
        is_possibly_3x3,
        is_possibly_4x4,
        is_possibly_5x5,
        is_possibly_6x6,
        is_possibly_midges,
        is_possibly_wings,
        is_probably_tcenters,
        is_possibly_xcenters,
        is_probably_midges,
    }: &Heuristics,
    mut f: impl FnMut(Case, &Tree),
) {
    if is_possibly_3x3 {
        if let Ok(tree) = parse_alg(3, ParseMode::Wca, line) {
            let mut cube = Cube::new_solved(3);
            tree.apply_inverse_to(&mut cube);
            if cube.corners.are_solved() {
                for cycle in collect_3cycle(&cube.edges) {
                    f(Case::Edge3Cycle(cycle), &tree);
                }
            }

            if cube.edges.are_solved() {
                for cycle in collect_3cycle(&cube.corners) {
                    f(Case::Corner3Cycle(cycle), &tree);
                }
            }

            for (corner_cycle, edge_cycle) in collect_2swap2swap(&cube.corners, &cube.edges) {
                f(Case::Corner2SwapEdge2Swap(corner_cycle, edge_cycle), &tree);
            }
        }
    }

    if is_possibly_4x4 {
        if let Ok(tree) = parse_alg(4, ParseMode::Wca, line) {
            let mut cube = Cube::new_solved(4);
            tree.apply_inverse_to(&mut cube);
            if cube.corners.are_solved() && cube.edges.are_solved() {
                if is_possibly_xcenters && cube.layers[0].wings.are_solved() {
                    for cycle in collect_3cycle(&cube.layers[0].xcenters) {
                        f(Case::XCenter3Cycle(cycle), &tree);
                    }
                }
                if is_possibly_wings && cube.layers[0].xcenters.are_solved() {
                    for cycle in collect_3cycle(&cube.layers[0].wings) {
                        f(
                            Case::Wing3Cycle(
                                cycle.map(|x| x.edge_sticker_considering_handedness()),
                            ),
                            &tree,
                        );
                    }
                    for swap in collect_2swap(&cube.layers[0].wings) {
                        f(Case::Wing2Swap(swap), &tree);
                    }
                }
            }
        }
    }

    if is_possibly_5x5 {
        if let Ok(tree) = parse_alg(5, ParseMode::Wca, line) {
            let mut cube = Cube::new_solved(5);
            tree.apply_inverse_to(&mut cube);
            if cube.corners.are_solved()
                && cube.layers[0].xcenters.are_solved()
                && cube.layers[0].wings.are_solved()
            {
                if cube.edges.are_solved() {
                    for cycle in collect_3cycle(&cube.layers[0].tcenters) {
                        f(Case::TCenter3Cycle(cycle), &tree);
                    }
                }
                if is_possibly_midges && cube.layers[0].tcenters.are_solved() {
                    for cycle in collect_3cycle(&cube.edges) {
                        f(Case::Midge3Cycle(cycle), &tree);
                    }
                }
            }
        }
    }

    if is_probably_midges || is_probably_tcenters {
        if let Ok(tree) = parse_alg(
            5,
            ParseMode::Wca,
            &line.replace('M', "m").replace('E', "e").replace('S', "s"),
        ) {
            let mut cube = Cube::new_solved(5);
            tree.apply_inverse_to(&mut cube);
            if cube.corners.are_solved()
                && cube.layers[0].xcenters.are_solved()
                && cube.layers[0].wings.are_solved()
            {
                if is_probably_tcenters && cube.edges.are_solved() {
                    for cycle in collect_3cycle(&cube.layers[0].tcenters) {
                        f(Case::TCenter3Cycle(cycle), &tree);
                    }
                }
                if is_probably_midges && cube.layers[0].tcenters.are_solved() {
                    for cycle in collect_3cycle(&cube.edges) {
                        f(Case::Midge3Cycle(cycle), &tree);
                    }
                }
            }
        }
    }

    if is_possibly_6x6 {
        if let Ok(tree) = parse_alg(6, ParseMode::Wca, line) {
            let mut cube = Cube::new_solved(6);
            tree.apply_inverse_to(&mut cube);
            if cube.corners.are_solved()
                && cube.edges.are_solved()
                && cube.layers[0].is_solved()
                && cube.layers[1].wings.are_solved()
                && cube.layers[1].xcenters.are_solved()
                && cube.layers[1].tcenters.are_solved()
            {
                let lobliques = &cube.layers[1].obliques[0].left;
                let robliques = &cube.layers[1].obliques[0].right;
                if robliques.are_solved() {
                    for cycle in collect_3cycle(lobliques) {
                        f(Case::LeftOblique3Cycle(cycle), &tree);
                    }
                }
                if lobliques.are_solved() {
                    for cycle in collect_3cycle(robliques) {
                        f(Case::RightOblique3Cycle(cycle), &tree);
                    }
                }
            }
        }
    }
}

fn collect_3cycle<P: Pieces>(pieces: &P) -> impl Iterator<Item = [P::Sticker; 3]> + '_ {
    P::Permutation::SOLVED
        .iter()
        .copied()
        .filter(|&buffer| {
            pieces.at(P::sticker(buffer, P::Orientation::GOOD))
                != P::sticker(buffer, P::Orientation::GOOD)
        })
        .filter_map(move |buffer| {
            let memo = memo(pieces, P::sticker(buffer, P::Orientation::GOOD));
            (memo.cycles.len() == 1
                && memo.parity.is_none()
                && memo.cycles[0][0] == memo.cycles[0].iter().copied().min().unwrap())
            .then(|| memo.cycles[0])
        })
}

fn collect_2swap<P: Pieces>(pieces: &P) -> impl Iterator<Item = [P::Sticker; 2]> + '_ {
    P::Permutation::SOLVED
        .iter()
        .copied()
        .filter(|&buffer| {
            pieces.at(P::sticker(buffer, P::Orientation::GOOD))
                != P::sticker(buffer, P::Orientation::GOOD)
        })
        .filter_map(move |buffer| {
            let memo = memo(pieces, P::sticker(buffer, P::Orientation::GOOD));
            memo.parity.filter(|_| memo.cycles.is_empty())
        })
}

fn collect_2swap2swap<'a, P: Pieces, Q: Pieces>(
    pieces: &'a P,
    qieces: &'a Q,
) -> impl Iterator<Item = ([P::Sticker; 2], [Q::Sticker; 2])> + 'a {
    collect_2swap(pieces)
        .flat_map(move |p_swap| collect_2swap(qieces).map(move |q_swap| (p_swap, q_swap)))
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
enum Source {
    Custom {
        url: String,
        name: String,
    },
    Spreadsheet {
        author: String,
        url: String,
        sheet_name: String,
    },
}
