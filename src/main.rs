use actix_cors::Cors;
use actix_web::{web, App, HttpServer};

mod api;
mod models;
mod repository;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let workflow_db = repository::database::Database::new();
    let app_data = web::Data::new(workflow_db);
    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .app_data(app_data.clone())
            .configure(api::api::config)
            .wrap(actix_web::middleware::Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
