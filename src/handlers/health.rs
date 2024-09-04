use actix_web::{HttpRequest, HttpResponse, Responder};

use crate::models::user::User;

pub async fn healthcheck(request: HttpRequest) -> impl Responder {
    dbg!(request);
    HttpResponse::Ok().append_header(("X-asd", "asd")).await
}

pub async fn index() -> impl Responder {
    "Welcome Anonymous!".to_owned()
}

pub async fn login(user: User) -> impl Responder {
    dbg!(user);
    HttpResponse::Ok()
}

pub async fn logout() -> impl Responder {
    HttpResponse::Ok()
}
