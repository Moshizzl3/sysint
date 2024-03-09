use crate::dtos::response_dto;
use crate::service::file_service::DataReader;
use actix_web::web;
use actix_web::web::Data;
use actix_web::{get, http::header, HttpResponse, Responder};
use bytes::Bytes;
use futures::stream::StreamExt;
use tokio::time::{interval, Duration};
use tokio_stream::wrappers::IntervalStream;

const  PATH_TO_FILES: &str = "/Users/mohamedibrahim/Desktop/2-semester.nosync/sysint/sysint/04_real_time_communicaton_part_II/SSE_example/sse_example/src/static/files/";

#[get("/sse")]
pub async fn get_csv_data() -> impl Responder {
    let file_reader = DataReader::new(PATH_TO_FILES);

    match file_reader.read_csv() {
        Ok(pokemon_dto) => {
            // Creating a stream
            let stream = IntervalStream::new(interval(Duration::from_secs(1))).map(move |_| {
                let data = format!("data: {}\n\n", serde_json::to_string(&pokemon_dto).unwrap());
                // Convert String to Bytes
                Result::<_, actix_web::Error>::Ok(Bytes::from(data))
            });

            HttpResponse::Ok()
                .insert_header(header::ContentType(mime::TEXT_EVENT_STREAM))
                .streaming(stream)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Error reading CSV: {}", e)),
    }
}
pub fn sse_router_config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("sse").service(get_csv_data));
}

