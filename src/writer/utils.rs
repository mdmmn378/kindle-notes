use polars::prelude::*;
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

pub struct Args {
    pub input: String,
    pub output: String,
}

pub fn extract_cli_args() -> Args {
    let args: Vec<String> = std::env::args().collect();
    let input = args[1].clone();
    let output = args[2].clone();
    println!("input: {}", input);
    println!("output: {}", output);
    Args { input, output }
}
#[allow(dead_code)]
pub fn to_df(titles: Vec<String>, timestamps: Vec<String>, texts: Vec<String>) -> DataFrame {
    let series_title = Series::new("title", titles);
    let series_timestamp = Series::new("timestamp", timestamps);
    let series_text = Series::new("text", texts);
    let df = DataFrame::new(vec![series_title, series_timestamp, series_text]).unwrap();
    df
}

pub fn read_file(args: &Args) -> Vec<String> {
    let path = Path::new(&args.input);
    let file = File::open(&path).unwrap();
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).unwrap();
    contents
        .split("==========")
        .map(|s| s.to_string())
        .filter(|s| s != &"\r\n")
        .collect()
}

pub fn parse_entry(text: String) -> Option<(String, String, String)> {
    let text = text.trim().to_string();
    let lines = text.lines().collect::<Vec<&str>>();
    let title = lines[0].to_string();
    let time = extract_timestamp(&text).unwrap();
    let text = lines[2..].join("");
    Some((title, time, text))
}

pub fn extract_timestamp(text: &str) -> Option<String> {
    let re =
        Regex::new(r"Added on (\w+, \w+ \d{1,2}, \d{4} \d{1,2}:\d{1,2}:\d{1,2} [AP]M)").unwrap();

    if let Some(captures) = re.captures(text) {
        let timestamp = captures.get(1).unwrap().as_str().to_string();
        return Some(timestamp);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_extract_with_re() {
        let text =
            "Your Highlight on Location 185-185 | Added on Monday, February 21, 2022 7:49:48 PM"
                .to_string();
        assert_eq!(
            extract_timestamp(&text).unwrap(),
            "Monday, February 21, 2022 7:49:48 PM"
        );
    }
}
