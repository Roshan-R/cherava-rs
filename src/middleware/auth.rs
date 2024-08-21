use crate::config::MC;
use actix_web::body::EitherBody;
use actix_web::{HttpRequest, HttpResponse};
use futures_util::FutureExt;
use magic_crypt::MagicCryptTrait;
use std::future::{ready, Ready};
use std::rc::Rc;

use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};

use actix_web::dev::forward_ready;
use futures_util::future::LocalBoxFuture;

pub struct Auth;

impl<S: 'static, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = AuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct AuthMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let (http_req, payload) = req.into_parts();
        let unauthorized_response = |http_request: HttpRequest| {
            ServiceResponse::new(
                http_request,
                HttpResponse::Unauthorized()
                    .finish()
                    .map_into_right_body::<B>(),
            )
        };

        let auth_header = match http_req.headers().get("Authorization") {
            Some(header) => header.to_str().ok(),
            None => return Box::pin(async move { Ok(unauthorized_response(http_req)) }),
        };

        // Extract and validate the token
        let token = match parse_token(auth_header) {
            Some(token) => token,
            None => return Box::pin(async move { Ok(unauthorized_response(http_req)) }),
        };

        let svc = self.service.clone();

        async move {
            if !validate_token(token).await {
                return Ok(unauthorized_response(http_req));
            }
            let res = svc
                .call(ServiceRequest::from_parts(http_req, payload))
                .await?;
            Ok(res.map_into_left_body())
        }
        .boxed_local()
    }
}

// Function to parse the token from the authorization header
fn parse_token(auth_header: Option<&str>) -> Option<String> {
    auth_header.and_then(|header| {
        if header.starts_with("Bearer ") {
            let token = &header["Bearer ".len()..];
            MC.decrypt_base64_to_string(token).ok()
        } else {
            None
        }
    })
}

// Function to validate the token with GitHub
async fn validate_token(token: String) -> bool {
    let client = reqwest::Client::new();
    client
        .get("https://api.github.com")
        .header("Authorization", format!("Bearer {}", token))
        .header("User-Agent", "Cherava")
        .send()
        .await
        .map_or(false, |response| response.status().is_success())
}
