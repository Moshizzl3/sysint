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

    pub fn read_xml(&self) -> Result<PokemonDTO, Box<dyn Error>> {
        let file_path = self.path.clone() + "pokemon.xml";
        let xml_content = fs::read_to_string(&file_path)?;
        let input_dto: PokemonInputXmlDTO = serde_xml_rs::from_str(&xml_content)?;

        // Convert to PokemonDTO
        let pokemon_dto = PokemonDTO {
            name: input_dto.name,
            level: input_dto.level,
            elements: input_dto.elements.element,
        };
        println!("{:?}", pokemon_dto);

        Ok(pokemon_dto)
    }

    pub fn read_yaml(&self) -> Result<PokemonDTO, Box<dyn Error>> {
        let file_path = self.path.clone() + "pokemon.yaml";
        let data = fs::read_to_string(file_path)?;

        let pokemon: PokemonDTO = serde_yaml::from_str(&data)?;
        println!("{:?}", pokemon);
        Ok(pokemon)
    }
}
