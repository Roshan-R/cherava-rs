use crate::config::MC;
use actix_session::Session;
use actix_web::web;
use actix_web::web::Redirect;
use actix_web::Responder;
use magic_crypt::MagicCryptTrait;
use serde_json::Value;

use crate::handlers::auth;
use auth::types::*;

pub async fn auth_callback(
    req_body: web::Query<AuthCallbackParams>,
    session: Session,
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

    let token: Value = res.json().await.unwrap();
    dbg!(&token);

    dbg!(session.entries());

    // let encrypted_token = MC.encrypt_str_to_base64(token.access_token);
    let encrypted_token =
        MC.encrypt_str_to_base64(token.get("access_token").unwrap().as_str().unwrap());

    // let res = client .get("https://api.github.com/user") .header("Authorization",
    // format!("BEARER {}", token.access_token)) .header("User-Agent", "Cherava") .send() .await
    // .unwrap();
    //
    // let data: Value = res.json().await.unwrap(); dbg!(&data);

    Redirect::to(format!(
        "http://localhost:8000?access_token={}",
        encrypted_token
    ))
}
