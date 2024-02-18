use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PokemonDTO {
    pub name: String,
    pub level: i8,
    pub elements: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct PokemonInputDTO {
    pub name: String,
    pub level: i8,
    pub elements: String,
}
