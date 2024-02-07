use clap::{App, Arg};
use csv::{ReaderBuilder, WriterBuilder};
use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::{Reader, Writer};
use serde_json::Value as JsonValue;
use std::collections::BTreeMap;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};

fn main() -> Result<(), Box<dyn Error>> {
    // get arguments from cli using clap crate
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

    let input_file = matches.value_of("input_file").unwrap();
    let output_format = matches.value_of("output_format").unwrap();

    let data = match load_file(input_file)? {
        Some(data) => data,
        None => return Err("Unsupported file format".into()),
    };

    convert_to_format(data, output_format)?;

    Ok(())
}

/// Loads file and return its content
fn load_file(file_path: &str) -> Result<Option<Vec<BTreeMap<String, String>>>, Box<dyn Error>> {
    if file_path.ends_with(".csv") {
        return read_csv_or_txt(file_path);
    } else if file_path.ends_with(".txt") {
        return read_csv_or_txt(file_path);
    } else if file_path.ends_with(".json") {
        return read_json(file_path);
    } else if file_path.ends_with(".xml") {
        return read_xml(file_path);
    } else if file_path.ends_with(".yaml") {
        return read_yaml(file_path);
    } else {
        return Ok(None);
    }
}

/// Converts file to specified format
fn convert_to_format(
    data: Vec<BTreeMap<String, String>>,
    format_type: &str,
) -> Result<(), Box<dyn Error>> {
    match format_type {
        "csv" => write_csv(&data, "output.csv")?,
        "txt" => write_txt(&data, "output.txt")?,
        "json" => write_json(&data, "output.json")?,
        "xml" => write_xml(&data, "output.xml")?,
        "yaml" => write_yaml(&data, "output.yaml")?,
        _ => return Err("Unsupported output format".into()),
    }

    Ok(())
}

/// Read csv or txt file and return content as a BTreeMap.
fn read_csv_or_txt(
    file_path: &str,
) -> Result<Option<Vec<BTreeMap<String, String>>>, Box<dyn Error>> {
    let delimiter = if file_path.ends_with(".txt") {
        b' '
    } else {
        b','
    };
    let mut rdr = ReaderBuilder::new()
        .delimiter(delimiter)
        .from_path(file_path)?;
    let headers = rdr.headers()?.clone();

    let mut records = Vec::new();
    for result in rdr.records() {
        let record = result?;
        let mut row = BTreeMap::new();
        for (header, field) in headers.iter().zip(record.iter()) {
            row.insert(header.to_string(), field.to_string());
        }
        records.push(row);
    }

    Ok(Some(records))
}

/// Read json file and return content as a BTreeMap.
/// We assume that we get an array
/// serde used here
fn read_json(file_path: &str) -> Result<Option<Vec<BTreeMap<String, String>>>, Box<dyn Error>> {
    let data = fs::read_to_string(file_path).expect("Unable to read file");

    let json: JsonValue = serde_json::from_str(&data).expect("JSON does not have correct format.");

    match json {
        JsonValue::Array(array) => {
            let mut result = Vec::new();

            for item in array {
                if let JsonValue::Object(obj) = item {
                    let mut map = BTreeMap::new();
                    for (key, value) in obj {
                        map.insert(key, value.to_string().trim_matches('"').to_string());
                    }
                    result.push(map);
                }
            }

            Ok(Some(result))
        }
        _ => Ok(None),
    }
}

/// Read xml file and return content as a BTreeMap.
/// We skip "event.name" called index if present
/// We assume that "event.name" is called "data" that contain data
/// quick_xml used here
fn read_xml(file_path: &str) -> Result<Option<Vec<BTreeMap<String, String>>>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let file_reader = BufReader::new(file);
    let mut reader = Reader::from_reader(file_reader);
    reader.trim_text(true);

    let mut buf = Vec::new();
    let mut records = Vec::new();
    let mut current_record = BTreeMap::new();
    let mut current_tag = None;

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                current_tag = Some(e.unescape_and_decode(&reader)?);
            }
            Ok(Event::Text(ref e)) => {
                if let Some(tag) = &current_tag {
                    if tag != "index" {
                        current_record.insert(tag.clone(), e.unescape_and_decode(&reader)?);
                    }
                }
            }
            Ok(Event::End(ref e)) => {
                if e.name() == b"row" {
                    records.push(current_record);
                    current_record = BTreeMap::new();
                }
                current_tag = None;
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(Box::new(e)),
            _ => (),
        }

        buf.clear();
    }

    Ok(Some(records))
}

/// Read yaml file and return content as a BTreeMap.
/// serde used here
fn read_yaml(file_path: &str) -> Result<Option<Vec<BTreeMap<String, String>>>, Box<dyn Error>> {
    let data = fs::read_to_string(file_path)?;

    let result: Vec<BTreeMap<String, String>> = serde_yaml::from_str(&data)?;

    Ok(Some(result))
}

/// write txt file
fn write_txt(data: &[BTreeMap<String, String>], output_format: &str) -> Result<(), Box<dyn Error>> {
    let mut wrt = File::create(output_format)?;

    // Specify the order of the columns
    let column_order = vec!["age", "name"]; // TODO: find a better way than hard coding
    let delimiter = " ";

    // Write the headers
    let header_line = column_order.join(delimiter) + "\n";
    wrt.write_all(header_line.as_bytes())?;

    // Write each row in the specified column order
    for row in data {
        let ordered_values: Vec<String> = column_order
            .iter()
            .map(|&key| row.get(key).cloned().unwrap_or_default())
            .collect();
        let line = ordered_values.join(delimiter) + "\n";
        wrt.write_all(line.as_bytes())?;
    }
    wrt.flush()?;

    Ok(())
}
/// write csv file
fn write_csv(
    data: &Vec<BTreeMap<String, String>>,
    output_format: &str,
) -> Result<(), Box<dyn Error>> {
    let mut wrt = WriterBuilder::new().from_path(output_format)?;

    // Specify the order of the columns
    let column_order = vec!["age", "name"]; // TODO: find a better way than hard coding

    // Write the headers in the specified order
    wrt.write_record(&column_order)?;

    let default_string = "".to_string();

    // Write each row in the specified column order
    for row in data {
        let ordered_values: Vec<&String> = column_order
            .iter()
            .map(|&key| row.get(key).unwrap_or(&default_string))
            .collect();
        wrt.write_record(ordered_values)?;
    }
    wrt.flush()?;
    Ok(())
}
/// write json file, we use serde here
fn write_json(
    data: &Vec<BTreeMap<String, String>>,
    output_format: &str,
) -> Result<(), Box<dyn Error>> {
    let file = File::create(output_format)?;
    serde_json::to_writer(file, data)?;

    Ok(())
}

/// write xml file
/// almost died making this 💀
fn write_xml(data: &[BTreeMap<String, String>], output_format: &str) -> Result<(), Box<dyn Error>> {
    let file = File::create(output_format)?;
    let writer = BufWriter::new(file);
    let mut writer = Writer::new(writer);

    // Start the root element
    writer.write_event(Event::Start(BytesStart::borrowed_name(b"data")))?;

    for record in data {
        // Start a row element
        writer.write_event(Event::Start(BytesStart::borrowed_name(b"row")))?;

        for (key, value) in record {
            let key = key.as_bytes();
            let value = value.as_bytes();

            // Start the element with the key name
            writer.write_event(Event::Start(BytesStart::borrowed_name(key)))?;

            // Write the value as text
            writer.write_event(Event::Text(BytesText::from_plain_str(std::str::from_utf8(
                value,
            )?)))?;

            // Close the element
            writer.write_event(Event::End(BytesEnd::borrowed(key)))?;
        }

        // Close the row element
        writer.write_event(Event::End(BytesEnd::borrowed(b"row")))?;
    }

    // Close the root element
    writer.write_event(Event::End(BytesEnd::borrowed(b"data")))?;

    Ok(())
}

/// write yaml file
/// using serde
fn write_yaml(
    data: &Vec<BTreeMap<String, String>>,
    output_format: &str,
) -> Result<(), Box<dyn Error>> {
    let file = File::create(output_format)?;
    serde_yaml::to_writer(file, data)?;

    Ok(())
}
