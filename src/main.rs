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

use config::CONFIG;

fn main() -> Result<(), std::io::Error> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let sentry_options = sentry::ClientOptions {
        release: sentry::release_name!(),
        ..Default::default()
    };
    let _guard = sentry::init((CONFIG.sentry_dsn.as_str(), sentry_options));

    // Start the actual mail function here
    actix_web::rt::System::new().block_on(async { startup().await })?;
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
