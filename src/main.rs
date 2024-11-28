use csv::{ReaderBuilder, WriterBuilder};
use std::env;
use std::fs;
use std::path::Path;

const OUTPUT_DELIMITER: u8 = b',';
const RAW_DELIMITER: u8 = b':';

fn concatenate_csv_files(
    input_folder: &str,
    output_file: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut writer = WriterBuilder::new()
        .delimiter(OUTPUT_DELIMITER)
        .from_path(output_file)?;
    let mut first_file = true;

    // Iterate through files in the folder
    for entry in fs::read_dir(input_folder)? {
        let entry = entry?;
        let path = entry.path();

        // Skip non-CSV files
        if path.extension().map_or(false, |ext| ext == "csv") {
            let mut reader = ReaderBuilder::new()
                .delimiter(RAW_DELIMITER)
                .from_path(&path)?;

            // Write headers only once
            if first_file {
                if let Ok(headers) = reader.headers() {
                    writer.write_record(headers)?;
                }
                first_file = false;
            }

            // Write the remaining rows
            for result in reader.records() {
                let record = result?;
                writer.write_record(&record)?;
            }
        }
    }

    writer.flush()?;
    Ok(())
}

fn main() {
    let input_folder = env::args().nth(1);
    let input_folder = input_folder.as_deref().unwrap_or("./tests/data/raw");

    let output_file = "./output.csv";

    if !Path::new(&input_folder).exists() {
        eprintln!("The input folder '{}' does not exist!", input_folder);
        std::process::exit(1);
    }

    match concatenate_csv_files(input_folder, output_file) {
        Ok(_) => println!("CSV files concatenated successfully!"),
        Err(e) => eprintln!("Error: {}", e),
    }
}
