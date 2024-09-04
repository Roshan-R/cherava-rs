use crate::config::MC;
use crate::repository::database::Database;
use crate::repository::schema::users;
use actix_session::SessionExt;
use actix_web::{dev::Payload, web, Error, FromRequest, HttpRequest};
use diesel::prelude::*;
use magic_crypt::MagicCryptTrait;
use serde::{Deserialize, Serialize};

use futures_util::future::Ready;
use futures_util::future::{err, ok};

#[derive(Debug, Queryable, Serialize, Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct User {
    pub user_id: i32,
    pub name: Option<String>,
    pub email: Option<String>,
    pub access_token: Option<String>,
}

impl FromRequest for User {
    type Error = Error;
    type Future = Ready<Result<Self, Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let internal_server_error =
            |mut error_string: Option<String>, is_unauthorized: bool| -> Self::Future {
                if error_string.is_none() {
                    error_string = Some(String::from("Oops something went wrong!"));
                }
                let error_string = error_string.clone().unwrap();
                let error = match is_unauthorized {
                    true => actix_web::error::ErrorUnauthorized(error_string),
                    false => actix_web::error::ErrorInternalServerError(error_string),
                };
                return err(error);
            };

        let db = match req.app_data::<web::Data<Database>>() {
            Some(db) => db.clone(),
            None => {
                return internal_server_error(Some(String::from("Cannot Access Database")), false)
            }
        };

        let session = req.get_session();
        let encrypted_token = match session.get::<String>("token").unwrap() {
            Some(tkn) => tkn,
            None => return internal_server_error(None, true),
        };

        let access_token = MC.decrypt_base64_to_string(&encrypted_token).ok().unwrap();
        match db.get_user_from_access_token(access_token) {
            Some(user) => ok(user),
            None => internal_server_error(None, true),
        }
    }
}
