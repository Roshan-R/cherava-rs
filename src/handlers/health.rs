use actix_web::HttpResponse;
use actix_web::Responder;

pub async fn healthcheck() -> impl Responder {
    HttpResponse::Ok()
}
