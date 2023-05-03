use crate::models::workflow;
use crate::repository::schema;

use workflow::{NewWorkflow, Workflow};

use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenvy::dotenv;
type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct Database {
    pool: DBPool,
}

impl Database {
    pub fn new() -> Self {
        dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool: DBPool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        Database { pool }
    }
    pub fn get_workflows_by_userid(&self, userid: &str) -> Option<Vec<Workflow>> {
        use schema::workflows::dsl::*;
        let results = workflows
            .filter(user.eq(userid))
            .load::<Workflow>(&mut self.pool.get().unwrap())
            .optional()
            .expect("Error loading workflows");
        results
    }

    pub fn create_workflow(
        &self,
        id: &str,
        user: &str,
        data: &str,
        selector: &str,
        cron: &str,
        lastupdated: &i64,
        url: &str,
        name: &str,
        email: &str,
    ) -> Option<Workflow> {
        let new_post = NewWorkflow {
            id,
            user,
            data,
            selector,
            cron,
            lastupdated,
            url,
            name,
            email,
        };
        use schema::workflows;

        diesel::insert_into(workflows::table)
            .values(&new_post)
            .get_result(&mut self.pool.get().unwrap())
            .optional()
            .expect("Error saving new workflow")
    }
}
