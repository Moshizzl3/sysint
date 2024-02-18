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

#[derive(Debug, Deserialize)]
pub struct PokemonInputXmlDTO {
    pub name: String,
    pub level: i8,
    pub elements: Elements,
}

#[derive(Debug, Deserialize)]
pub struct Elements {
    #[serde(rename = "element", default)]
    pub element: Vec<String>,
}
