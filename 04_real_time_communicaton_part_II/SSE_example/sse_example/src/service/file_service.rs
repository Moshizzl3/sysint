use crate::dtos::pokemon_dtos::{PokemonDTO, PokemonInputDTO, PokemonInputXmlDTO};
use std::error::Error;
use std::fs;
use std::fs::File;

pub struct DataReader {
    path: String,
}

impl DataReader {
    pub fn new(path: &str) -> Self {
        return DataReader {
            path: path.to_string(),
        };
    }
    /// Read CSV file and return a single PokemonDTO.
    pub fn read_csv(&self) -> Result<PokemonDTO, Box<dyn Error>> {
        let file_path = self.path.clone() + "pokemon.csv";
        let file = File::open(file_path)?;

        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(file);

        let result = rdr.deserialize::<PokemonInputDTO>().next();
        if let Some(record) = result {
            let temp_pokemon = record?;
            let elements_vec = temp_pokemon.elements.split(" ").map(String::from).collect();

            let pokemon = PokemonDTO {
                name: temp_pokemon.name,
                level: temp_pokemon.level,
                elements: elements_vec,
            };

            return Ok(pokemon);
        } else {
            return Err("No data found in CSV".into());
        }
    }
}
