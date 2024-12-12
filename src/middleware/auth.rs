use crate::config::MC;
use crate::controllers::database::Database;
use actix_session::SessionExt;
use actix_web::body::EitherBody;
use actix_web::web;
use actix_web::{HttpRequest, HttpResponse};
use core::panic;
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
        let db = match req.app_data::<web::Data<Database>>() {
            Some(db) => db.clone(),
            None => panic!("kevin malone"),
        };

        let (http_req, payload) = req.into_parts();
        let unauthorized_response = |http_request: HttpRequest| {
            ServiceResponse::new(
                http_request,
                HttpResponse::Unauthorized()
                    .finish()
                    .map_into_right_body::<B>(),
            )
        };

        let session = http_req.get_session();

        let encrypted_token = match session.get::<String>("token").unwrap() {
            Some(tkn) => tkn,
            None => return Box::pin(async move { Ok(unauthorized_response(http_req)) }),
        };

        let access_token = MC.decrypt_base64_to_string(&encrypted_token).ok().unwrap();

        // TODO: make this async
        if db.get_user_from_access_token(access_token).is_none() {
            return Box::pin(async move { Ok(unauthorized_response(http_req)) });
        }

        let svc = self.service.clone();
        async move {
            let res = svc
                .call(ServiceRequest::from_parts(http_req, payload))
                .await?;
            Ok(res.map_into_left_body())
        }
        .boxed_local()
    }
}
