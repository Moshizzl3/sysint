use serde::{Deserialize, Serialize};

use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify, OpenApi, ToSchema,
};

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ResponseData<T> {
    pub data: T,
}
