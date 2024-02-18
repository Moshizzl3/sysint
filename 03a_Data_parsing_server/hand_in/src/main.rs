use crate::handlers::files::files_router_config;
use actix_web::{middleware::Logger, App, HttpServer};

mod dtos;
mod handlers;
mod service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .configure(files_router_config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
