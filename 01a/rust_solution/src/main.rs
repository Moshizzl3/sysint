use clap::{App, Arg};
use csv::{ReaderBuilder, WriterBuilder};
use dtos::pokemon_dtos;
use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::{Reader, Writer};
use serde_json::Value as JsonValue;
use std::collections::BTreeMap;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};

mod dtos;
mod service;

fn main() {
    const  PATH_TO_FILES: &str = "/Users/mohamedibrahim/Desktop/2-semester.nosync/sysint/sysint/01a/rust_solution/src/static/files/";

    let file_reader = service::file_service::DataReader::new(PATH_TO_FILES);

    match file_reader.read_yaml() {
        Ok(pokemon_dto) => {
            println!("{:#?}", pokemon_dto)
        }
        Err(e) => println!("Some error orrcured: {:?}", e),
    }
}
