use crate::{config::CONFIG, models::user::User};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct AuthCallbackParams {
    pub code: String,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct AccessTokenParams {
    client_id: String,
    client_secret: String,
    code: String,
    redirect_uri: String,
}

impl AccessTokenParams {
    pub fn new(code: String) -> AccessTokenParams {
        return AccessTokenParams {
            client_id: CONFIG.client_id.clone(),
            client_secret: CONFIG.client_secret.clone(),
            code,
            redirect_uri: CONFIG.redirect_uri.clone(),
        };
    }
}

#[derive(Deserialize, Debug, Serialize)]
pub struct AccessToken {
    pub access_token: String,
    scope: String,
    token_type: String,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct UserResp {
    pub email: Option<String>,
    pub id: i32,
    pub name: String,
    pub access_token: Option<String>,
}

impl Into<User> for UserResp {
    fn into(self) -> User {
        User {
            user_id: self.id,
            name: Some(self.name),
            email: self.email,
            access_token: self.access_token,
        }
    }
}
