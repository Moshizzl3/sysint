use serde::{Deserialize, Serialize};

use crate::dtos::pokemon_dtos::PokemonDTO;
use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify, OpenApi, ToSchema,
};

#[derive(Serialize, Deserialize, ToSchema)]
#[aliases(ResponseDataString = ResponseData<String>, ResponseDataPokemonDTO = ResponseData<PokemonDTO>)]
pub struct ResponseData<T> {
    pub data: T,
}
