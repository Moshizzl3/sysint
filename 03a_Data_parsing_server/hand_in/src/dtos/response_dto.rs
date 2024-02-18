use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ResponseData<T> {
    pub data: T,
}
