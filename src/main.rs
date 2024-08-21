mod config;
mod handlers;
mod middleware;
mod models;
mod repository;
mod routes;

mod server;
use server::server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    server().await
}
