use actix_identity::Identity;
use actix_session::Session;
use actix_web::web::{Data, Query, Redirect};
use actix_web::{HttpMessage, HttpRequest, Responder};

use crate::config::MC;
use crate::handlers::auth;
use crate::models::user::User;
use crate::repository::database::Database;

use auth::types::*;
use magic_crypt::MagicCryptTrait;

pub async fn auth_callback(
    db: Data<Database>,
    req_body: Query<AuthCallbackParams>,
    session: Session,
    request: HttpRequest,
) -> impl Responder {
    let url = String::from("https://github.com/login/oauth/access_token");
    let params = AccessTokenParams::new(req_body.code.clone());
    let client = reqwest::Client::new();

    let res = client
        .post(url)
        .json(&params)
        .header("Accept", "application/json")
        .send()
        .await
        .unwrap();

    let token: AccessToken = res.json().await.unwrap();
    let encrypted_token = MC.encrypt_str_to_base64(token.access_token.as_str());

    let mut resp: UserResp = client
        .get("https://api.github.com/user")
        .header("Authorization", format!("Bearer {}", token.access_token))
        .header("User-Agent", "Cherava")
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    resp.access_token = Some(token.access_token);
    let user: User = resp.into();
    dbg!(&user);
    let user = db.get_or_create_user(user);

    session.insert("user_id", user.user_id).unwrap();
    session.insert("token", &encrypted_token).unwrap();

    Identity::login(&request.extensions(), user.user_id.to_string()).unwrap();
    Redirect::to("http://localhost:8000")
}
