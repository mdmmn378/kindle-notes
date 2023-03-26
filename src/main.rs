mod writer;

use writer::exporter::*;
use writer::utils::*;

fn main() {
    let args = extract_cli_args();
    let entries = read_file(&args);

    let mut titles: Vec<String> = Vec::new();
    let mut timestamps: Vec<String> = Vec::new();
    let mut texts: Vec<String> = Vec::new();

    for entry in entries.clone() {
        if let Some((title, time, text)) = parse_entry(entry) {
            titles.push(title);
            timestamps.push(time);
            texts.push(text);
        } else {
            println!("Could not extract book information from text.");
        }
    }

    export_to_xlsx(
        args.output,
        titles.clone(),
        timestamps.clone(),
        texts.clone(),
    );
    println!("🎉 Exported to xlsx file.");
}
