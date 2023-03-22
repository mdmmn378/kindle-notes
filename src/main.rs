use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

fn list_of_hashmaps_to_json(list: Vec<HashMap<String, String>>) -> bool {
    let mut json = String::new();
    json.push_str("[");
    for map in list {
        json.push_str("{");
        for (key, value) in map {
            json.push_str(&format!("\"{}\": \"{}\",", key, value));
        }
        json.pop();
        json.push_str("},");
    }
    json.pop();
    json.push_str("]");
    // write to file
    let path = Path::new("output.json");
    let mut file = File::create(&path).unwrap();
    file.write_all(json.as_bytes()).unwrap();
    true
}

fn main() {
    let path = Path::new("My Clippings.txt");
    let file = File::open(&path).unwrap();
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).unwrap();
    let mut lines = contents.lines();
    let mut book_info: Vec<HashMap<String, String>> = Vec::new();
    while let Some(line) = lines.next() {
        if line.contains("==========") {
            let tmp = lines.next();
            if tmp.is_none() {
                break;
            }
            let book = tmp.unwrap().to_string();
            let note = lines.next().unwrap().to_string();
            let date = lines.next().unwrap().to_string();
            let location = lines.next().unwrap().to_string();
            let mut map = HashMap::new();
            map.insert("book".to_string(), book);
            map.insert("note".to_string(), note);
            map.insert("date".to_string(), date);
            map.insert("location".to_string(), location);
            book_info.push(map);
        }
    }
    println!("{:?}", book_info);
    list_of_hashmaps_to_json(book_info);
}

//TODO
// 1. Add error handling
// 2. Add tests
// 3. Add documentation
// 4. Add command line arguments
// 5. Add support for other formats
// 6. Add regex date parsing
