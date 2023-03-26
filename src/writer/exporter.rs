use polars::prelude::*;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[allow(dead_code)]
pub fn export_df_to_csv(df: DataFrame) -> bool {
    let path = Path::new("output.csv");
    let mut file = File::create(&path).unwrap();
    CsvWriter::new(&mut file)
        .has_header(true)
        .with_delimiter(b',')
        .finish(&mut df.clone())
        .unwrap();
    true
}

#[allow(dead_code)]
pub fn export_to_json(titles: Vec<String>, timestamps: Vec<String>, texts: Vec<String>) -> bool {
    let path = Path::new("output.json");
    let mut file = File::create(&path).unwrap();
    let mut json = String::new();
    json.push_str("{");
    json.push_str("\"titles\": [");
    for title in titles {
        json.push_str(&format!("\"{}\",", title));
    }
    json.pop();
    json.push_str("],");
    json.push_str("\"timestamps\": [");
    for timestamp in timestamps {
        json.push_str(&format!("\"{}\",", timestamp));
    }
    json.pop();
    json.push_str("],");
    json.push_str("\"texts\": [");
    for text in texts {
        json.push_str(&format!("\"{}\",", text));
    }
    json.pop();
    json.push_str("]");
    json.push_str("}");
    file.write_all(json.as_bytes()).unwrap();
    true
}

pub fn export_to_xlsx(
    file_name: String,
    titles: Vec<String>,
    timestamps: Vec<String>,
    texts: Vec<String>,
) {
    let mut book = umya_spreadsheet::new_file();
    let _ = book.new_sheet("Sheet1");

    book.get_sheet_by_name_mut("Sheet1")
        .unwrap()
        .get_column_dimension_mut("B")
        .set_width(200.0);

    book.get_sheet_by_name_mut("Sheet1")
        .unwrap()
        .get_column_dimension_mut("A")
        .set_width(100.0);

    book.get_sheet_by_name_mut("Sheet1")
        .unwrap()
        .get_cell_mut(format!("A{}", 1))
        .set_value_from_string("Title");

    book.get_sheet_by_name_mut("Sheet1")
        .unwrap()
        .get_cell_mut(format!("B{}", 1))
        .set_value_from_string("Text");

    book.get_sheet_by_name_mut("Sheet1")
        .unwrap()
        .get_cell_mut(format!("C{}", 1))
        .set_value_from_string("Timestamp");

    for (i, title) in titles.iter().enumerate() {
        book.get_sheet_by_name_mut("Sheet1")
            .unwrap()
            .get_cell_mut(format!("A{}", i + 2))
            .set_value_from_string(title);
    }

    for (i, timestamp) in timestamps.iter().enumerate() {
        book.get_sheet_by_name_mut("Sheet1")
            .unwrap()
            .get_cell_mut(format!("C{}", i + 2))
            .set_value_from_string(timestamp);
    }

    for (i, text) in texts.iter().enumerate() {
        book.get_sheet_by_name_mut("Sheet1")
            .unwrap()
            .get_cell_mut(format!("B{}", i + 2))
            .set_value_from_string(text);
    }

    let path = std::path::Path::new(file_name.as_str());
    let _ = umya_spreadsheet::writer::xlsx::write(&book, path);
}

#[test]
fn test_export_to_xlsx() {
    let titles = vec!["title1".to_string(), "title2".to_string()];
    let timestamps = vec!["timestamp1".to_string(), "timestamp2".to_string()];
    let texts = vec!["text1".to_string(), "text2".to_string()];
    export_to_xlsx("test.xlsx".to_string(), titles, timestamps, texts);
}
