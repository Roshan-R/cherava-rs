use crate::config::CONFIG;
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
