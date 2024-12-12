use crate::repository::schema::workflows;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Insertable, Queryable, Clone)]
#[diesel(table_name = workflows)]
pub struct Workflow {
    pub id: String,
    pub user_id: i32,
    pub data: String,
    pub selector: String,
    pub cron: String,
    pub lastupdated: Option<i64>,
    pub url: String,
}
