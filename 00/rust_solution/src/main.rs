use clap::{App, Arg};
use csv::ReaderBuilder;
// use quick_xml::events::Event;
// use quick_xml::Reader;
// use serde::{Deserialize, Serialize};
// use serde_json::Value as JsonValue;
// use serde_yaml::Value as YamlValue;
use std::collections::HashMap;
use std::error::Error;
// use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("File Converter")
        .version("1.0")
        .author("Your Name")
        .about("Converts file formats")
        .arg(
            Arg::with_name("input_file")
                .short("i")
                .long("input")
                .value_name("FILE")
                .help("Sets the input file to use")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("output_format")
                .short("o")
                .long("output")
                .value_name("FORMAT")
                .help("Sets the output file format")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let input_file: &str = matches.value_of("input_file").unwrap();
    let output_format: &str = matches.value_of("output_format").unwrap();

    let data: Vec<HashMap<String, String>> = match load_file(input_file)? {
        Some(data) => data,
        None => return Err("Unsupported file format".into()),
    };

    convert_to_format(data, output_format)?;

    Ok(())
}

fn load_file(file_path: &str) -> Result<Option<Vec<HashMap<String, String>>>, Box<dyn Error>> {
    if file_path.ends_with(".csv") {
        read_csv(file_path)
    } else {
        Ok(None)
    }
}

fn read_csv(file_path: &str) -> Result<Option<Vec<HashMap<String, String>>>, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new().from_path(file_path)?;
    let headers = rdr.headers()?.clone();

    let mut records = Vec::new();
    for result in rdr.records() {
        let record = result?;
        let mut row = HashMap::new();
        for (header, field) in headers.iter().zip(record.iter()) {
            row.insert(header.to_string(), field.to_string());
        }
        records.push(row);
    }

    Ok(Some(records))
}

fn convert_to_format(
    data: Vec<HashMap<String, String>>,
    format_type: &str,
) -> Result<(), Box<dyn Error>> {
    match format_type {
        "csv" => write_csv(&data, "output.csv")?,
        _ => return Err("Unsupported output format".into()),
    }

    Ok(())
}

fn write_csv(data: &Vec<HashMap<String, String>>, file_path: &str) -> Result<(), Box<dyn Error>> {
    println!("{:?}", data);
    Ok(())
}

// TODO: Implement read_json, read_xml, read_yaml, read_txt
// TODO: Implement write_csv, write_json, write_xml, write_yaml, write_txt
