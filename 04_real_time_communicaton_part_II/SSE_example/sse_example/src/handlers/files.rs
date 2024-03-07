use crate::dtos::response_dto;
use crate::service::file_service::DataReader;
use actix_web::{get, web, HttpResponse, Responder};
use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify, OpenApi, ToSchema,
};

const  PATH_TO_FILES: &str = "/Users/mohamedibrahim/Desktop/2-semester.nosync/sysint/sysint/03a_Data_parsing_server/hand_in/src/static/files/";

#[utoipa::path(
    path="/files/csv",
    responses(
        (status = 200, description = "Hello from api 1", body = ResponseDataPokemonDTO)
    )
)]
#[get("/csv")]
pub async fn get_csv_data() -> impl Responder {
    let file_reader = DataReader::new(PATH_TO_FILES);

    match file_reader.read_csv() {
        Ok(pokemon_dto) => {
            HttpResponse::Ok().json(response_dto::ResponseData { data: pokemon_dto })
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Error reading CSV: {}", e)),
    }
}

// #[get("/json")]
// async fn get_json_data() -> impl Responder {
//     let file_reader = DataReader::new(PATH_TO_FILES);

//     match file_reader.read_json() {
//         Ok(pokemon_dtos) => {
//             HttpResponse::Ok().json(response_dto::ResponseData { data: pokemon_dtos })
//         }
//         Err(e) => HttpResponse::InternalServerError().body(format!("Error reading CSV: {}", e)),
//     }
// }

// #[get("/xml")]
// async fn get_xml_data() -> impl Responder {
//     let file_reader = DataReader::new(PATH_TO_FILES);

//     match file_reader.read_xml() {
//         Ok(pokemon_dtos) => {
//             HttpResponse::Ok().json(response_dto::ResponseData { data: pokemon_dtos })
//         }
//         Err(e) => HttpResponse::InternalServerError().body(format!("Error reading CSV: {}", e)),
//     }
// }

// #[get("/yaml")]
// async fn get_yaml_data() -> impl Responder {
//     let file_reader = DataReader::new(PATH_TO_FILES);

//     match file_reader.read_yaml() {
//         Ok(pokemon_dtos) => {
//             HttpResponse::Ok().json(response_dto::ResponseData { data: pokemon_dtos })
//         }
//         Err(e) => HttpResponse::InternalServerError().body(format!("Error reading CSV: {}", e)),
//     }
// }

pub fn files_router_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("files").service(get_csv_data), // .service(get_json_data)
                                                   // .service(get_xml_data)
                                                   // .service(get_yaml_data),
    );
}
