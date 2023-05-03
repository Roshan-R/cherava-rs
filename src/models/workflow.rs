use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct Workflow {
    pub id: String,
    pub user: Option<String>,
    pub data: Option<String>,
    pub selector: Option<String>,
    pub cron: Option<String>,
    pub lastupdated: Option<i64>,
    pub url: Option<String>,
    pub name: Option<String>,
    pub email: Option<String>,
}

use crate::repository::schema::workflows;

#[derive(Insertable)]
#[diesel(table_name = workflows)]
pub struct NewWorkflow<'a> {
    pub id: &'a str,
    pub user: &'a str,
    pub data: &'a str,
    pub selector: &'a str,
    pub cron: &'a str,
    pub lastupdated: &'a i64,
    pub url: &'a str,
    pub name: &'a str,
    pub email: &'a str,
}
