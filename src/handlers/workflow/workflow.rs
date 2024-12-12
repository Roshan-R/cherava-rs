use crate::controllers::database::Database;
use crate::models::user::User;
use actix_web::{web, HttpResponse, Responder};

use crate::handlers::workflow;
use std::time::{SystemTime, UNIX_EPOCH};
use workflow::types::*;

pub async fn w_by_uid(db: web::Data<Database>, user: User) -> impl Responder {
    let workflows = db.get_workflows_by_userid(user.user_id);
    dbg!(&workflows);
    match workflows {
        Some(w) => HttpResponse::Ok().json(w),
        None => HttpResponse::NotFound().body("Workflow not found"),
    }
}

pub async fn create_new_workflow(
    db: web::Data<Database>,
    data: web::Json<NewWorkflowReq>,
    user: User,
) -> impl Responder {
    let now = SystemTime::now();

    // Calculate the duration since the Unix epoch
    let duration_since_epoch = now.duration_since(UNIX_EPOCH).unwrap();

    // Convert the duration to seconds
    let timestamp: i64 = duration_since_epoch.as_secs().try_into().unwrap();

    let workflow = db.create_workflow(
        data.id.to_owned(),
        user.user_id,
        data.data.to_owned(),
        data.selector.to_owned(),
        data.cron.to_owned(),
        timestamp,
        data.url.to_owned(),
    );
    let s = match workflow {
        Some(_w) => StatusResp { worked: true },
        None => StatusResp { worked: false },
    };
    HttpResponse::Ok().json(s)
}
