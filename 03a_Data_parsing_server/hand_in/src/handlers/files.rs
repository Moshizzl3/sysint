use crate::dtos::{pokemon_dtos, response_dto};
use actix_web::{get, web, HttpResponse, Responder};

#[get("/csv")]
async fn get_csv_data() -> impl Responder {
    let pokemon_dto = pokemon_dtos::PokemonDTO {
        name: String::from("hello"),
        level: 5, // example level
        elements: vec!["Fire".to_string(), "Water".to_string()],
    };
    HttpResponse::Ok().json(response_dto::ResponseData { data: pokemon_dto })
}

#[get("/json")]
async fn get_json_data() -> impl Responder {
    let pokemon_dto = pokemon_dtos::PokemonDTO {
        name: String::from("hello"),
        level: 5, // example level
        elements: vec!["Fire".to_string(), "Water".to_string()],
    };
    HttpResponse::Ok().json(response_dto::ResponseData { data: pokemon_dto })
}

#[get("/xml")]
async fn get_xml_data() -> impl Responder {
    let pokemon_dto = pokemon_dtos::PokemonDTO {
        name: String::from("hello"),
        level: 5, // example level
        elements: vec!["Fire".to_string(), "Water".to_string()],
    };
    HttpResponse::Ok().json(response_dto::ResponseData { data: pokemon_dto })
}

#[get("/yaml")]
async fn get_yaml_data() -> impl Responder {
    let pokemon_dto = pokemon_dtos::PokemonDTO {
        name: String::from("hello"),
        level: 5, // example level
        elements: vec!["Fire".to_string(), "Water".to_string()],
    };
    HttpResponse::Ok().json(response_dto::ResponseData { data: pokemon_dto })
}

pub fn files_router_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("files")
            .service(get_csv_data)
            .service(get_json_data)
            .service(get_xml_data)
            .service(get_yaml_data),
    );
}
