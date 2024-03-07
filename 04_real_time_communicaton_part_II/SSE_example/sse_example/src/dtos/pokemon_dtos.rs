use serde::{Deserialize, Serialize};
use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify, OpenApi, ToSchema,
};


#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct PokemonDTO {
    pub name: String,
    pub level: i16,
    pub elements: Vec<String>,
}

#[derive(Deserialize, Debug, ToSchema)]
pub struct PokemonInputDTO {
    pub name: String,
    pub level: i16,
    pub elements: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct PokemonInputXmlDTO {
    pub name: String,
    pub level: i16,
    pub elements: Elements,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct Elements {
    #[serde(rename = "element", default)]
    pub element: Vec<String>,
}
