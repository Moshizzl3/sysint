use crate::dtos::pokemon_dtos::{PokemonDTO, PokemonInputDTO};
use csv::{ReaderBuilder, WriterBuilder};
use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::{Reader, Writer};
use serde_json::Value as JsonValue;
use std::collections::BTreeMap;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};

const  PATH_TO_FILES: &str = "/Users/mohamedibrahim/Desktop/2-semester.nosync/sysint/sysint/03a_Data_parsing_server/hand_in/src/static/files/pokemon.csv";

pub struct DataReader {
    path: String,
}

impl DataReader {
    pub fn new(path: &str) -> Self {
        return DataReader {
            path: path.to_string(),
        };
    }
    /// Read CSV file and return a vector of PokemonDTO.
    pub fn read_csv(&self) -> Result<Vec<PokemonDTO>, Box<dyn Error>> {
        let file_path = self.path.clone() + "pokemon.csv";
        let file = File::open(file_path)?;

        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(true) // Set to false if your CSV doesn't have headers
            .from_reader(file);

        let mut pokemons: Vec<PokemonDTO> = Vec::new();

        for result in rdr.deserialize::<PokemonInputDTO>() {
            let temp_pokemon = result?;
            let elements_vec = temp_pokemon.elements.split(" ").map(String::from).collect();

            let pokemon: PokemonDTO = PokemonDTO {
                name: temp_pokemon.name,
                level: temp_pokemon.level,
                elements: elements_vec,
            };

            pokemons.push(pokemon);
        }

        println!("{:?}", pokemons);
        Ok(pokemons)
    }

    /// Read JSON file and return content as a PokemonDTO.
    pub fn read_json(&self) -> Result<PokemonDTO, Box<dyn Error>> {
        let file_path = self.path.clone() + "pokemon.json";
        // Read the file to a string
        let data = fs::read_to_string(file_path)?;

        // Deserialize the JSON string into PokemonDTO
        let pokemon: PokemonDTO = serde_json::from_str(&data)?;

        println!("{:?}", pokemon);
        Ok(pokemon)
    }

    // Placeholder for future implementations
    pub fn read_xml(&self) {
        // Implementation for XML...
    }
    pub fn read_yaml(&self) {
        // Implementation for YAML...
    }
}
