use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct ScrapeResp {
    pub d: String,
}

#[derive(Deserialize)]
pub struct GetDataReq {
    pub id: String,
}

#[derive(Deserialize)]
pub struct NewWorkflowReq {
    pub id: String,
    pub user: String,
    pub data: String,
    pub selector: String,
    pub cron: String,
    pub lastupdated: i64,
    pub url: String,
    pub name: String,
    pub email: String,
}

#[derive(Deserialize)]
pub enum ScrapeType {
    Text,
    Html,
}

#[derive(Deserialize)]
pub struct ScrapeReq {
    pub url: String,
    pub selector: String,
    pub r#type: ScrapeType,
}

#[derive(Serialize)]
pub struct StatusResp {
    pub worked: bool,
}
