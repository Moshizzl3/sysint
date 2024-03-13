use crate::dtos::response_dto;
use crate::service::file_service::DataReader;
use actix_web::{get, web, HttpResponse, Responder};
use utoipa::ToSchema;

const  PATH_TO_FILES: &str = "/Users/mohamedibrahim/Desktop/2-semester.nosync/sysint/sysint/04_real_time_communicaton_part_II/SSE_example/sse_example/src/static/files/";

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

pub fn files_router_config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("files").service(get_csv_data));
}
