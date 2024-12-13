use crate::handlers::auth::auth::auth_callback;
use crate::handlers::health::{index, login, logout};
use crate::handlers::{health, workflow};

use actix_web::web;
use workflow::scrape::handle_scrape;
use workflow::workflow::{create_new_workflow, w_by_uid};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .service(
                web::scope("/example")
                    .route("/", web::get().to(index))
                    .route("/login", web::post().to(login))
                    .route("/logout", web::post().to(logout)),
            )
            .route("/health", web::get().to(health::healthcheck))
            .route("/scrape", web::post().to(handle_scrape))
            .service(web::scope("/auth").route("/callback", web::get().to(auth_callback)))
            .service(
                web::scope("/workflows")
                    .route("/get", web::get().to(w_by_uid))
                    .route("/new", web::post().to(create_new_workflow)),
            ),
    );
}
