mod config;
mod controllers;
mod handlers;
mod middleware;
mod models;
mod repository;
mod routes;
mod server;

use controllers::database::Database;

use controllers::cron_scheduler;
use cron_scheduler::scheduler::schedule_workflows;
use dotenv::dotenv;
use server::server;
use tokio_cron_scheduler::JobScheduler;

fn main() -> Result<(), std::io::Error> {
    dotenv().ok();
    let _guard = sentry::init((
        std::env::var("SENTRY_DSN").unwrap(),
        sentry::ClientOptions {
            release: sentry::release_name!(),
            ..Default::default()
        },
    ));
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async { startup().await })?;
    Ok(())
}

async fn startup() -> Result<(), std::io::Error> {
    let db = Database::new();

    let sched = JobScheduler::new().await.unwrap();
    schedule_workflows(&sched, db.clone())
        .await
        .expect("Error in creating a cron schedules");
    sched.start().await.unwrap();

    server(db).await
}
