use crate::handlers::auth::auth::auth_callback;
use crate::handlers::{health, workflow};
use crate::middleware::auth::Auth as AuthMiddleware;

use actix_web::web;
use workflow::workflow::{create_new_workflow, scrape, w_by_uid};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        // /api/v1 routes
        .service(
            web::scope("/api/v1")
                // Lock down routes with AUTH Middleware
                .wrap(AuthMiddleware)
                // Healthcheck
                .route("/health", web::get().to(health::healthcheck))
                // Auth routes
                .service(web::scope("/auth").route("/callback", web::get().to(auth_callback)))
                // Workflow routes
                .service(
                    web::scope("/workflow")
                        .route("/get", web::post().to(w_by_uid))
                        .route("/scrape", web::post().to(scrape))
                        .route("/new", web::post().to(create_new_workflow)),
                ),
        );
}
