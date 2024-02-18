use crate::handlers::files::files_router_config;
use crate::service::file_service;
use actix_web::{middleware::Logger, App, HttpServer};

mod dtos;
mod handlers;
mod service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let file_reader = file_service::DataReader::new("/Users/mohamedibrahim/Desktop/2-semester.nosync/sysint/sysint/03a_Data_parsing_server/hand_in/src/static/files/");

    file_reader.read_csv();
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .configure(files_router_config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
