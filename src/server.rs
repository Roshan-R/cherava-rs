use log::info;

use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer};

use crate::config::CONFIG;
use crate::routes;

async fn not_found() -> HttpResponse {
    HttpResponse::NotFound().body("Not Found, 404 Error")
}

pub async fn server() -> std::io::Result<()> {
    //let workflow_db = repository::database::Database::new(); let app_data =
    //web::Data::new(workflow_db);

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    info!(
        "Connected
        to port {} ",
        CONFIG.port
    );

    HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            // .wrap(
            //     SessionMiddleware::builder(CookieSessionStore::default(), Key::generate())
            //         .cookie_secure(false)
            //         .build(),
            // )
            //.app_data(app_data.clone())
            .configure(routes::routes)
            .default_service(web::route().to(not_found))
            .wrap(Cors::permissive())
    })
    .bind(("127.0.0.1", CONFIG.port))?
    .run()
    .await
}
