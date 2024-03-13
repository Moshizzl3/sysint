use serde::{Deserialize, Serialize};

use crate::dtos::pokemon_dtos::PokemonDTO;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
#[aliases(ResponseDataString = ResponseData<String>, ResponseDataPokemonDTO = ResponseData<PokemonDTO>)]
pub struct ResponseData<T> {
    pub data: T,
}
