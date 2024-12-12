use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct ScrapeResp {
    pub d: String,
}

#[derive(Deserialize)]
pub struct NewWorkflowReq {
    pub id: String,
    pub data: String,
    pub selector: String,
    pub url: String,
    pub cron: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ScrapeType {
    Text,
    Html,
}

#[derive(Deserialize)]
pub struct ScrapeReq {
    pub url: String,
    pub selector: String,
    #[serde(default = "default_type")]
    pub r#type: ScrapeType,
}

fn default_type() -> ScrapeType {
    ScrapeType::Text
}

#[derive(Serialize)]
pub struct StatusResp {
    pub worked: bool,
}
