use csv::{ReaderBuilder, WriterBuilder};
use std::env;
use std::fs;
use std::path::Path;

const RAW_DELIMITER: u8 = b':';
const OUTPUT_DELIMITER: u8 = b';';

const COL_FILENAME: &str = "Filename";
const COL_SIZE: &str = "Size";
const COL_WIDTH: &str = "Width";
const COL_OBJECT_NAME: &str = "IPTC:Object Name";
const COL_SUP_CATEGORY: &str = "IPTC:Sup. Category";
const COL_SOURCE: &str = "IPTC:Source";
const OPTIONAL_COLS: &[&str] = &["IPTC:Caption"];

const CATEGORY: &str = "MQB - Iconotheque";
const SOURCE: &str = "Fichier produit par Arkhenum";
const MIN_SIZE: f64 = 2.0; // in Mio
const MAX_SIZE: f64 = 4.0; // in Mio
const WIDTH_DPI: &str = "3200";

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

fn validate_csv(file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // Use ReaderBuilder to configure the reader with a custom delimiter
    let mut reader = ReaderBuilder::new()
        .delimiter(OUTPUT_DELIMITER)
        .has_headers(true) // Indicate that the CSV file has headers
        .from_path(file_path)?;

    let headers = reader.headers()?.clone();

    // Get column indices based on header names
    let col_filename = headers.iter().position(|h| h == COL_FILENAME).unwrap();
    let col_size = headers.iter().position(|h| h == COL_SIZE).unwrap();
    let col_width = headers.iter().position(|h| h == COL_WIDTH).unwrap();
    let col_object_name = headers.iter().position(|h| h == COL_OBJECT_NAME).unwrap();
    let col_sup_category = headers.iter().position(|h| h == COL_SUP_CATEGORY).unwrap();
    let col_source = headers.iter().position(|h| h == COL_SOURCE).unwrap();

    // Iterate through rows
    for (line_number, result) in reader.records().enumerate() {
        let record = result?;
        let mut errors = Vec::new();

        for (i, value) in record.iter().enumerate() {
            if value.trim().is_empty() && !OPTIONAL_COLS.contains(&headers.get(i).unwrap()) {
                errors.push(format!("Field '{}' is empty", headers.get(i).unwrap()));
            }
        }

        let size = record[col_size].trim();
        if let Some(size_value) = size.strip_suffix(" Mio") {
            if let Ok(size_parsed) = size_value.parse::<f64>() {
                if size_parsed < MIN_SIZE {
                    errors.push(format!(
                        "Size '{} Mio' is below the minimum '{} Mio'",
                        size_parsed, MIN_SIZE
                    ));
                } else if size_parsed > MAX_SIZE {
                    errors.push(format!(
                        "Size '{} Mio' is above the maximum '{} Mio'",
                        size_parsed, MAX_SIZE
                    ));
                }
            } else {
                errors.push(format!("Invalid size format: {}", size_value));
            }
        }

        let width = record[col_width].trim();
        if width != WIDTH_DPI {
            errors.push(format!(
                "Width is '{}', it should be '{}'",
                width, WIDTH_DPI
            ));
        }

        let object_name = record[col_object_name].trim();
        let filename = record[col_filename].trim();
        let filename_prefix = filename.split('_').next().unwrap_or("");
        if filename_prefix != object_name {
            errors.push(format!(
                "Object name '{}' does not match filename '{}'",
                object_name.trim(),
                filename
            ));
        }

        let sup_category = record[col_sup_category].trim();
        if sup_category != CATEGORY {
            errors.push(format!(
                "Sup category is '{}', it should be '{}'",
                sup_category, CATEGORY
            ));
        }

        let source = record[col_source].trim();
        if source != SOURCE {
            errors.push(format!("Source is '{}', it should be '{}'", source, SOURCE));
        }

        if !errors.is_empty() {
            println!(
                "Line {}",
                line_number + 2, // Add 2 to account for header and 0-based index
            );
            for error in errors {
                println!("\t* {}", error);
            }
            println!("{:?}", record);
            println!();
        }
    }

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

    match validate_csv(Path::new(output_file)) {
        Ok(_) => println!("CSV file validated successfully!"),
        Err(e) => eprintln!("Error: {}", e),
    }
}
