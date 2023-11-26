use calamine::{open_workbook_auto, DataType, Reader};
use std::{collections::HashMap, path::PathBuf};

fn main() -> anyhow::Result<()> {
    let names_by_sheet = csv::Reader::from_path("google.csv")?
        .deserialize::<(String, String)>()
        .map(|x| x.map(|(a, b)| (b, a)))
        .collect::<csv::Result<HashMap<String, String>>>()?;

    let mut writer = csv::Writer::from_path("fields.csv")?;

    for ent in std::fs::read_dir("google")? {
        let ent = ent?;
        let file_name = PathBuf::from(ent.file_name());
        let is_excel_file = file_name
            .extension()
            .is_some_and(|x| x == "xls" || x == "xlsx");
        if !is_excel_file {
            continue;
        }

        let workbook_id = file_name
            .file_stem()
            .unwrap()
            .to_string_lossy()
            .into_owned();

        let mut book = open_workbook_auto(ent.path())?;
        for (sheet_name, sheet) in book.worksheets() {
            for (x, y, content) in sheet.cells() {
                if let DataType::String(text) = content {
                    writer.write_record([
                        &workbook_id,
                        &names_by_sheet[&workbook_id],
                        &sheet_name,
                        &x.to_string(),
                        &y.to_string(),
                        text,
                    ])?;
                }
            }
        }
    }

    writer.flush()?;
    Ok(())
}
