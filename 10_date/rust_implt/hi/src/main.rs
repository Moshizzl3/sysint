use actix_cors::Cors;
use actix_web::{
    get, http::header, middleware::Logger, options, App, HttpResponse, HttpServer, Responder,
};
use chrono;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub status: &'static str,
    pub message: String,
}

#[get("/date")]
async fn get_date() -> impl Responder {
    let datel = chrono::Local::now(); // Corrected$ this line
    println!("{:?}", datel);
    let test = HttpResponse::Ok()
        .append_header(("ngrok-skip-browser-warning", "69420"))
        .json(Response {
            status: "Success",
            message: datel.to_string(),
        });

    print!("{:?}", test);
    test
}

// #[get("/external")]
// async fn get_external() -> impl Responder {
//     const url: &str = "https://cc19-195-249-146-100.ngrok-free.app/mo/html";

//     match reqwest::get(url).await {
//         Ok(res) => {
//             match res.text().await {
//                 Ok(text) => {
//                     // If successful, return the text as the response body
//                     HttpResponse::build(StatusCode::OK)
//                         .content_type("text/html; charset=utf-8")
//                         .body(text)
//                 }
//                 Err(e) => {
//                     HttpResponse::InternalServerError().body(format!("Error parsing JSON: {}", e))
//                 }
//             }
//         }
//         Err(e) => HttpResponse::InternalServerError().body(format!("Error fetching data: {}", e)),
//     }
// }

#[get("/external")]
async fn get_external() -> impl Responder {
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("./static/index.html"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .service(get_date)
            .service(get_external)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
