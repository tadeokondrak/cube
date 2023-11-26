use cube::{CornerSticker, Cube, EdgeOrientation, EdgeSticker, Face, WingSticker};
use cube_bld::{memo, memo_centers, Pieces, Sticker};
use cube_notation::ParseMode;
use std::{collections::HashMap, fmt::Write};
use wasm_bindgen::prelude::*;

#[cfg(test)]
mod tests;

fn ordinal_suffix(n: usize) -> &'static str {
    match (n % 10, n % 100) {
        (_, 11 | 12 | 13) => "th",
        (1, _) => "st",
        (2, _) => "nd",
        (3, _) => "rd",
        _ => "th",
    }
}

fn apply_alg(cube: &mut Cube, alg: &str) {
    if let Ok(tree) = cube_notation::parse_alg(cube.n, ParseMode::Wca, alg) {
        tree.apply_to(cube);
    }
}

#[wasm_bindgen]
pub fn display(n: u16, scramble: &str) -> String {
    let mut cube = Cube::new_solved(n);
    apply_alg(&mut cube, scramble);

    let mut s = String::new();
    let height = 3 * usize::from(cube.n);
    let width = 4 * usize::from(cube.n);
    let face_offsets = [(1, 0), (0, 1), (1, 1), (2, 1), (3, 1), (1, 2)];

    s.push_str(&format!(
        r#"<div style="background-color: black; display: grid; grid-template-columns: repeat({width}, 1fr); grid-template-rows: repeat({height}, 1fr); width: 600px; height: 400px">"#
    ));

    for face_index in 0..6 {
        let face = Face::from_index(face_index);
        let (x_offset, y_offset) = face_offsets[face_index];
        for x in 0..(n | 1) {
            for y in 0..(n | 1) {
                let adj_x = x as i16 - cube.n as i16 / 2;
                let adj_y = -(y as i16 - cube.n as i16 / 2);
                if n % 2 == 0 && (adj_x == 0 || adj_y == 0) {
                    continue;
                }

                let x = if n % 2 == 0 && x > n / 2 { x - 1 } else { x };
                let y = if n % 2 == 0 && y > n / 2 { y - 1 } else { y };

                let color = cube.color_at(face, adj_x, adj_y);
                let color_name = match color {
                    Face::U => "white",
                    Face::L => "orange",
                    Face::F => "green",
                    Face::R => "red",
                    Face::B => "blue",
                    Face::D => "yellow",
                };

                s.push_str(&format!(
                    r#"<div style="background-color: {color_name}; grid-row-start: {y}; grid-row-end: {y}; grid-column-start: {x}; grid-column-end: {x}"></div>"#,
                    x = x_offset * cube.n + x + 1,
                    y = y_offset * cube.n + y + 1,
                ));
            }
        }
    }

    s.push_str("</div>");

    s
}

#[wasm_bindgen]
#[allow(clippy::too_many_arguments)]
pub fn memorize(
    n: u16,
    scramble: &str,
    lettering: &str,
    edge_buffer: usize,
    corner_buffer: usize,
    wing_buffer: usize,
    xcenter_buffer: usize,
    tcenter_buffer: usize,
    loblique_buffer: usize,
    roblique_buffer: usize,
) -> String {
    console_error_panic_hook::set_once();

    let buffers = Buffers {
        edges: EdgeSticker::from_index(edge_buffer),
        corners: CornerSticker::from_index(corner_buffer),
        wings: WingSticker::from_index(wing_buffer),
        xcenters: CornerSticker::from_index(xcenter_buffer),
        tcenters: EdgeSticker::from_index(tcenter_buffer),
        lobliques: EdgeSticker::from_index(loblique_buffer),
        robliques: EdgeSticker::from_index(roblique_buffer),
    };

    let mut cube = Cube::new_solved(n);
    apply_alg(&mut cube, scramble);

    let lettering = lettering.chars().enumerate().collect::<HashMap<_, _>>();

    let mut s = String::new();

    fn write_spaced(
        out: &mut String,
        lettering: &HashMap<usize, char>,
        iter: impl Iterator<Item = usize>,
    ) {
        for (i, letter) in iter.enumerate() {
            if i != 0 && i % 2 == 0 {
                out.push(' ');
            }
            out.push(lettering[&letter]);
        }
    }

    fn write_unspaced(
        out: &mut String,
        lettering: &HashMap<usize, char>,
        iter: impl Iterator<Item = usize>,
    ) {
        out.extend(iter.into_iter().map(|letter| (lettering[&letter])));
    }

    if n % 2 != 0 {
        let edge_memo = memo(&cube.edges, buffers.edges);
        s.push_str("Edges: ");
        write_spaced(
            &mut s,
            &lettering,
            edge_memo
                .cycles
                .iter()
                .flat_map(|x| &x[1..])
                .map(|x| x.index()),
        );
        if let Some([_, parity]) = edge_memo.parity {
            s.push(' ');
            s.push(lettering[&parity.index()]);
        }

        if !edge_memo.twists.is_empty() {
            s.push_str("; flip ");
            for &(flip, _) in &edge_memo.twists {
                let flip =
                    EdgeSticker::from_permutation_and_orientation(flip, EdgeOrientation::Good);
                s.push(lettering[&flip.index()]);
            }
        }

        s.push('\n');
    }

    if n > 1 {
        let corner_memo = memo(&cube.corners, buffers.corners);
        s.push_str("Corners: ");
        write_spaced(
            &mut s,
            &lettering,
            corner_memo
                .cycles
                .iter()
                .flat_map(|x| &x[1..])
                .map(|x| x.index()),
        );

        if let Some([_, parity]) = corner_memo.parity {
            s.push(' ');
            s.push(lettering[&parity.index()]);
        }

        if !corner_memo.twists.is_empty() {
            s.push_str("; twist ");
            write_unspaced(
                &mut s,
                &lettering,
                corner_memo
                    .twists
                    .iter()
                    .copied()
                    .map(|(piece, orientation)| {
                        CornerSticker::from_permutation_and_orientation(piece, -orientation).index()
                    }),
            );
        }
    }

    for (i, layer) in cube.layers.iter().enumerate() {
        write!(&mut s, "\n\nLayer {}", { i + 1 }).unwrap();
        let xcenter_memo = memo_centers(&layer.xcenters, buffers.xcenters);
        let tcenter_memo = memo_centers(&layer.tcenters, buffers.tcenters);
        let wing_memo = memo(&layer.wings, buffers.wings);

        s.push_str("\nX-centers: ");
        write_spaced(
            &mut s,
            &lettering,
            xcenter_memo
                .cycles
                .iter()
                .flat_map(|x| &x[1..])
                .map(|x| x.index()),
        );
        if let Some([_, parity]) = xcenter_memo.parity {
            s.push(' ');
            s.push(lettering[&parity.index()]);
        }

        if n % 2 != 0 {
            s.push_str("\nT-centers: ");
            write_spaced(
                &mut s,
                &lettering,
                tcenter_memo
                    .cycles
                    .iter()
                    .flat_map(|x| &x[1..])
                    .map(|x| x.index()),
            );
            if let Some([_, parity]) = tcenter_memo.parity {
                s.push(' ');
                s.push(lettering[&parity.index()]);
            }
        }

        s.push_str("\nWings: ");
        write_spaced(
            &mut s,
            &lettering,
            wing_memo
                .cycles
                .iter()
                .flat_map(|x| &x[1..])
                .map(|x| x.edge_sticker_considering_handedness().index()),
        );
        if let Some([_, parity]) = wing_memo.parity {
            s.push(' ');
            s.push(lettering[&parity.edge_sticker_considering_handedness().index()]);
        }

        for (i, oblique_pair) in layer.obliques.iter().enumerate() {
            let l_oblique_memo = memo_centers(&oblique_pair.left, buffers.lobliques);
            let r_oblique_memo = memo_centers(&oblique_pair.right, buffers.robliques);

            write!(
                &mut s,
                "\n{}{} left obliques: ",
                i + 1,
                ordinal_suffix(i + 1)
            )
            .unwrap();
            write_spaced(
                &mut s,
                &lettering,
                l_oblique_memo
                    .cycles
                    .iter()
                    .flat_map(|x| &x[1..])
                    .map(|x| x.index()),
            );
            write!(
                &mut s,
                "\n{}{} right obliques: ",
                i + 1,
                ordinal_suffix(i + 1)
            )
            .unwrap();
            write_spaced(
                &mut s,
                &lettering,
                r_oblique_memo
                    .cycles
                    .iter()
                    .flat_map(|x| &x[1..])
                    .map(|x| x.index()),
            );
        }
    }

    s
}

#[wasm_bindgen]
#[allow(clippy::too_many_arguments)]
pub fn analyze(
    _n: u16,
    scrambles: &str,
    memo: &str,
    _lettering: &str,
    edge_buffer: usize,
    corner_buffer: usize,
    wing_buffer: usize,
    xcenter_buffer: usize,
    tcenter_buffer: usize,
    loblique_buffer: usize,
    roblique_buffer: usize,
) -> String {
    console_error_panic_hook::set_once();

    let buffers = Buffers {
        edges: EdgeSticker::from_index(edge_buffer),
        corners: CornerSticker::from_index(corner_buffer),
        wings: WingSticker::from_index(wing_buffer),
        xcenters: CornerSticker::from_index(xcenter_buffer),
        tcenters: EdgeSticker::from_index(tcenter_buffer),
        lobliques: EdgeSticker::from_index(loblique_buffer),
        robliques: EdgeSticker::from_index(roblique_buffer),
    };

    let scrambles = scrambles.split('\n').collect::<Vec<_>>();
    let mut res = String::new();
    for cube_memo in memo
        .split("\n\n")
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
    {
        let (cube, scramble, score) = scrambles
            .iter()
            .map(|scramble| (scramble, execute_memo(&buffers, scramble, cube_memo)))
            .map(|(scramble, cube)| {
                let score = score_cube(&cube);
                (cube, scramble, score)
            })
            .max_by_key(|&(.., score)| score)
            .unwrap();
        writeln!(
            &mut res,
            "memo: \n{cube_memo}\nscramble: {scramble}\nscore: {score:?}\nsolved: {:?}\n",
            cube.corners.are_solved() && cube.edges.are_solved()
        )
        .unwrap();
    }

    res
}

struct Buffers {
    edges: EdgeSticker,
    corners: CornerSticker,
    wings: WingSticker,
    xcenters: CornerSticker,
    tcenters: EdgeSticker,
    lobliques: EdgeSticker,
    robliques: EdgeSticker,
}

fn execute_memo(buffers: &Buffers, scramble: &str, memo: &str) -> Cube {
    let mut cube = Cube::new_solved(3);
    apply_alg(&mut cube, scramble);

    for line in memo.lines() {
        if let Some(edges) = line.strip_prefix("e ").or(line.strip_prefix("edges ")) {
            exec_memo(
                buffers.edges,
                &mut cube.edges,
                &parse_memo::<EdgeSticker>(edges),
            );
        }
        if let Some(corners) = line.strip_prefix("c ").or(line.strip_prefix("corners ")) {
            exec_memo(
                buffers.corners,
                &mut cube.corners,
                &parse_memo::<CornerSticker>(corners),
            );
        }
    }
    // w(ings)
    // x(centers)
    // t(centers)
    // l(obliques)
    // r(obliques)
    cube
}

fn exec_memo<P: Pieces>(buffer: P::Sticker, pieces: &mut P, memo: &[P::Sticker]) {
    for pair in memo.chunks(2) {
        if pair.len() == 1 {
            pieces.cycle(&[buffer, pair[0]], 1);
            continue;
        } else {
            pieces.cycle(&[buffer, pair[0], pair[1]], 1);
        }
    }
}

fn parse_memo<S: Sticker>(memo: &str) -> Vec<S> {
    let mut res = Vec::new();
    for c in memo.chars() {
        match c.to_ascii_uppercase() {
            ' ' => continue,
            'A'..='Z' => {
                res.push(Sticker::from_index((c as u8 - b'A').into()));
            }
            _ => panic!(),
        }
    }
    res
}

fn score_cube(cube: &Cube) -> u16 {
    score_pieces(&cube.corners) + score_pieces(&cube.edges)
}

fn score_pieces<P: Pieces>(pieces: &P) -> u16 {
    let mut solved_count = 0;
    let mut permuted_count = 0;
    let mut oriented_count = 0;
    for &piece in P::Sticker::SOLVED {
        let oriented = P::sticker_orientation(pieces.at(piece)) == P::sticker_orientation(piece);
        let permuted = P::sticker_orientation(pieces.at(piece)) == P::sticker_orientation(piece);
        solved_count += (oriented && permuted) as u16;
        oriented_count += oriented as u16;
        permuted_count += permuted as u16;
    }
    solved_count + permuted_count + oriented_count
}
