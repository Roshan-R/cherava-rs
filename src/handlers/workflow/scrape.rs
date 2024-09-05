use crate::controllers::scraper::scrape;
use crate::handlers::workflow::types::{ScrapeReq, ScrapeResp};
use actix_web::{web, HttpResponse, Responder};

pub async fn handle_scrape(data: web::Json<ScrapeReq>) -> impl Responder {
    let selected: String = scrape(&data.url, &data.selector).await.unwrap();
    if selected.is_empty() {
        return HttpResponse::NotFound().body("Could not scrape");
    }

    HttpResponse::Ok().json(ScrapeResp { d: selected })
}
