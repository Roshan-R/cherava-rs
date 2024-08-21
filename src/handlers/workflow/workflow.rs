use crate::repository::database::Database;
use actix_web::{web, HttpResponse, Responder};

use crate::handlers::workflow;
use workflow::types::*;

use scraper::{Html, Selector};

pub async fn w_by_uid(db: web::Data<Database>, data: web::Json<GetDataReq>) -> impl Responder {
    let workflows = db.get_workflows_by_userid(&data.id.as_str());
    dbg!(&workflows);
    match workflows {
        Some(w) => HttpResponse::Ok().json(w),
        None => HttpResponse::NotFound().body("Workflow not found"),
    }
}

pub async fn scrape(data: web::Json<ScrapeReq>) -> impl Responder {
    let html = reqwest::get(&data.url).await.unwrap().text().await.unwrap();
    let document = Html::parse_document(html.as_str());
    let selector = Selector::parse(&data.selector).unwrap();

    let selected: String = match &data.r#type {
        ScrapeType::Text => document
            .select(&selector)
            .flat_map(|el| el.text())
            .collect(),
        ScrapeType::Html => document.select(&selector).next().unwrap().html(),
    };

    if selected.is_empty() {
        HttpResponse::NotFound().body("Could not scrape")
    } else {
        HttpResponse::Ok().json(ScrapeResp { d: selected })
    }
}

pub async fn create_new_workflow(
    db: web::Data<Database>,
    data: web::Json<NewWorkflowReq>,
) -> impl Responder {
    let workflow = db.create_workflow(
        data.id.as_str(),
        data.user.as_str(),
        data.data.as_str(),
        data.selector.as_str(),
        data.cron.as_str(),
        &data.lastupdated,
        data.url.as_str(),
        data.name.as_str(),
        data.email.as_str(),
    );
    dbg!(&workflow);
    let s = match workflow {
        Some(_w) => StatusResp { worked: true },
        None => StatusResp { worked: false },
    };
    HttpResponse::Ok().json(s)
}
