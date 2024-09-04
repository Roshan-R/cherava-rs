use actix_cors::Cors;
use actix_identity::IdentityMiddleware;
use actix_session::config::PersistentSession;
use actix_session::storage::CookieSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::Key;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer};

use crate::config::CONFIG;
use crate::repository::database::Database;
use crate::routes;

use log::info;

async fn not_found() -> HttpResponse {
    HttpResponse::NotFound().body("Not Found, 404 Error")
}

pub async fn server(db: Database) -> std::io::Result<()> {
    let app_data = web::Data::new(db);

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let session_secret_key = Key::from(
        "this is a simple session key that I have created? 
            How are you my dude this is the best and worst project I have writter"
            .as_bytes(),
    );

    info!("Connected to port {} ", CONFIG.port);
    HttpServer::new(move || {
        App::new()
            .wrap(IdentityMiddleware::default())
            .wrap(
                SessionMiddleware::builder(
                    CookieSessionStore::default(),
                    session_secret_key.clone(),
                )
                .cookie_secure(false)
                .cookie_http_only(false)
                .session_lifecycle(
                    PersistentSession::default()
                        .session_ttl(actix_web::cookie::time::Duration::weeks(2)),
                )
                .build(),
            )
            .wrap(Logger::default())
            .app_data(app_data.clone())
            .configure(routes::routes)
            .default_service(web::route().to(not_found))
            .wrap(Cors::permissive())
    })
    .bind(("127.0.0.1", CONFIG.port))?
    .run()
    .await
}
