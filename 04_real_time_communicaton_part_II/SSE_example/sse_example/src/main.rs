use crate::dtos::pokemon_dtos::PokemonDTO;
use crate::dtos::response_dto::{ResponseDataPokemonDTO, ResponseDataString};
use crate::handlers::{external::external_router_config, files::files_router_config};
use actix_web::{get, middleware::Logger, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify, OpenApi, ToSchema,
};
use utoipa_swagger_ui::SwaggerUi;

mod dtos;
mod handlers;
mod service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    #[derive(OpenApi)]
    #[openapi(
        paths(handlers::files::get_csv_data),
        components(schemas(ResponseDataString, ResponseDataPokemonDTO, PokemonDTO))
    )]

    struct ApiDoc;
    let openapi = ApiDoc::openapi();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
            .configure(files_router_config)
            .configure(external_router_config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
