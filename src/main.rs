use actix_cors::Cors;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;

mod api;
mod models;
mod repository;

use log::info;

#[derive(Serialize)]
pub struct Response {
    pub message: String,
}

#[get("/health")]
async fn healthcheck() -> impl Responder {
    let response = Response {
        message: "Everything is working fine".to_string(),
    };
    HttpResponse::Ok().json(response)
}

async fn not_found() -> HttpResponse {
    let response = Response {
        message: "Resource not found".to_string(),
    };
    HttpResponse::NotFound().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let workflow_db = repository::database::Database::new();
    let app_data = web::Data::new(workflow_db);
    let port: u16 = std::env::var("PORT")
        .unwrap_or("8080".to_string())
        .parse()
        .unwrap();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    info!("Connected to port {} ", port);

    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .app_data(app_data.clone())
            .configure(api::api::config)
            .service(healthcheck)
            .default_service(web::route().to(not_found))
            .wrap(cors)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
