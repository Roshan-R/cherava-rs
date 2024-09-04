mod config;
mod controllers;
mod handlers;
mod middleware;
mod models;
mod repository;
mod routes;
mod server;

use crate::repository::database::Database;

use controllers::cron_scheduler;
use cron_scheduler::scheduler::schedule_workflows;
use server::server;
use tokio_cron_scheduler::JobScheduler;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Database::new();

    let sched = JobScheduler::new().await.unwrap();
    schedule_workflows(&sched, db.clone()).await.unwrap();
    sched.start().await.unwrap();

    server(db).await
}
