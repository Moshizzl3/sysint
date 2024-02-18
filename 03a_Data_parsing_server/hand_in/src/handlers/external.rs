use crate::dtos::pokemon_dtos::PokemonDTO;
use crate::dtos::{pokemon_dtos, response_dto};
use crate::service::file_service::DataReader;
use actix_web::{get, web, HttpResponse, Responder};
use reqwest;

const  PATH_TO_FILES: &str = "/Users/mohamedibrahim/Desktop/2-semester.nosync/sysint/sysint/03a_Data_parsing_server/hand_in/src/static/files/";
const BASE_URL: &str = "http://localhost:8000/api/file";

#[get("/csv")]
async fn get_csv_data_external() -> impl Responder {
    let url = BASE_URL.to_string() + "/csv";
    match reqwest::get(url).await {
        Ok(resp) => match resp.json::<response_dto::ResponseData<PokemonDTO>>().await {
            Ok(pokemon_dto) => HttpResponse::Ok().json(pokemon_dto),
            Err(e) => {
                HttpResponse::InternalServerError().body(format!("Error parsing JSON: {}", e))
            }
        },
        Err(e) => HttpResponse::InternalServerError().body(format!("Error fetching data: {}", e)),
    }
}

#[get("/json")]
async fn get_json_data_external() -> impl Responder {
    let url = BASE_URL.to_string() + "/json";
    match reqwest::get(url).await {
        Ok(resp) => match resp.json::<response_dto::ResponseData<PokemonDTO>>().await {
            Ok(pokemon_dto) => HttpResponse::Ok().json(pokemon_dto),
            Err(e) => {
                HttpResponse::InternalServerError().body(format!("Error parsing JSON: {}", e))
            }
        },
        Err(e) => HttpResponse::InternalServerError().body(format!("Error fetching data: {}", e)),
    }
}

#[get("/xml")]
async fn get_xml_data_external() -> impl Responder {
    let url = BASE_URL.to_string() + "/xml";
    match reqwest::get(url).await {
        Ok(resp) => match resp.json::<response_dto::ResponseData<PokemonDTO>>().await {
            Ok(pokemon_dto) => HttpResponse::Ok().json(pokemon_dto),
            Err(e) => {
                HttpResponse::InternalServerError().body(format!("Error parsing JSON: {}", e))
            }
        },
        Err(e) => HttpResponse::InternalServerError().body(format!("Error fetching data: {}", e)),
    }
}

#[get("/yaml")]
async fn get_yaml_data_external() -> impl Responder {
    let url = BASE_URL.to_string() + "/yaml";
    match reqwest::get(url).await {
        Ok(resp) => match resp.json::<response_dto::ResponseData<PokemonDTO>>().await {
            Ok(pokemon_dto) => HttpResponse::Ok().json(pokemon_dto),
            Err(e) => {
                HttpResponse::InternalServerError().body(format!("Error parsing JSON: {}", e))
            }
        },
        Err(e) => HttpResponse::InternalServerError().body(format!("Error fetching data: {}", e)),
    }
}

pub fn external_router_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("external")
            .service(get_csv_data_external)
            .service(get_json_data_external)
            .service(get_xml_data_external)
            .service(get_yaml_data_external),
    );
}
