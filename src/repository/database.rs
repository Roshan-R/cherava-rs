use crate::config::CONFIG;
use crate::models::{user, workflow};
use crate::repository::schema;

use user::User;
use workflow::Workflow;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;

type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Debug, Clone)]
pub struct Database {
    pool: DBPool,
}

impl Database {
    pub fn new() -> Self {
        dotenv().ok();
        let manager = ConnectionManager::<PgConnection>::new(CONFIG.database_url.clone());
        let pool: DBPool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        Database { pool }
    }

    pub fn get_or_create_user(&self, user: User) -> User {
        use schema::users::dsl::*;
        let ser = users
            .filter(user_id.eq(user.user_id))
            .first::<User>(&mut self.pool.get().unwrap());

        if ser.is_ok() {
            let old_user: User = ser.unwrap();
            if old_user.access_token != user.access_token {
                diesel::update(users.filter(user_id.eq(old_user.user_id)))
                    .set(access_token.eq(user.access_token))
                    .execute(&mut self.pool.get().unwrap())
                    .unwrap();
            }
            return old_user;
        }

        let res_user = diesel::insert_into(users)
            .values(&user)
            .get_result::<User>(&mut self.pool.get().unwrap())
            .unwrap();
        return res_user;
    }

    pub fn get_user_from_access_token(&self, token: String) -> Option<User> {
        use schema::users::dsl::*;
        return users
            .filter(access_token.eq(token))
            .get_result::<User>(&mut self.pool.get().unwrap())
            .ok();
    }

    pub fn get_workflows_by_userid(&self, userid: i32) -> Option<Vec<Workflow>> {
        use schema::workflows::dsl::*;
        let results = workflows
            .filter(user_id.eq(userid))
            .load::<Workflow>(&mut self.pool.get().unwrap())
            .optional()
            .expect("Error loading workflows");
        results
    }

    pub fn get_all_workflows(&self) -> Vec<Workflow> {
        use schema::workflows::dsl::*;
        let results = workflows
            .load::<Workflow>(&mut self.pool.get().unwrap())
            .expect("Could not get all the workflows in the db");

        results
    }

    pub fn update_workflow(&self, w: Workflow) -> Result<(), diesel::result::Error> {
        use schema::workflows::dsl::*;
        diesel::update(workflows.filter(id.eq(w.id)))
            .set(data.eq(w.data))
            .execute(&mut self.pool.get().unwrap())?;
        Ok(())
    }

    pub fn create_workflow(
        &self,
        id: String,
        user_id: i32,
        data: String,
        selector: String,
        cron: String,
        lastupdated: i64,
        url: String,
    ) -> Option<Workflow> {
        let new_post = Workflow {
            id,
            user_id,
            data,
            selector,
            cron,
            lastupdated: Some(lastupdated),
            url,
        };
        use schema::workflows;

        diesel::insert_into(workflows::table)
            .values(&new_post)
            .get_result(&mut self.pool.get().unwrap())
            .optional()
            .expect("Error saving new workflow")
    }
}
