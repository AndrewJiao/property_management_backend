use crate::const_value::SETTINGS;
use crate::tools::jwt::{create_jwt_token_cookie, AppJwtToken, TokenOperation, JWT_TOKEN_KEY};
use actix::fut::{ready, Ready};
use actix_web::body::EitherBody;
use actix_web::dev::forward_ready;
use actix_web::{dev::{Service, ServiceRequest, ServiceResponse, Transform}, Error, HttpResponse};
use futures_util::future::LocalBoxFuture;
use log::info;
use crate::tools::jwt::TokenOperation::Fail;

pub struct JWTMiddleware;

impl<S, B> Transform<S, ServiceRequest> for JWTMiddleware
where
    S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = JWTHandler<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JWTHandler { service }))
    }
}
pub struct JWTHandler<S> {
    service: S,
}


impl<S, B> Service<ServiceRequest> for JWTHandler<S>
where
    S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;
    forward_ready!(service);
    fn call(&self, req: ServiceRequest) -> Self::Future {
        info!("jwt middleware call for uri {}", req.uri());
        if !ignore_path(req.path()) {
            let operation = req.cookie(JWT_TOKEN_KEY)
                .and_then(|cookie| {
                    let a = AppJwtToken::verify_token_str(cookie.value());
                    Some(a)
                }).unwrap_or_default();

            match operation {
                TokenOperation::Success => {
                    let service_fun = self.service.call(req);
                    Box::pin(async move {
                        let result = service_fun.await.map(|e| e.map_into_left_body())?;
                        Ok(result)
                    })
                }
                Fail => {
                    let error_res = HttpResponse::Unauthorized().finish().map_into_right_body();
                    Box::pin(async { Ok(req.into_response(error_res)) })
                }
                TokenOperation::SuccessAndRenew(new_token) => {
                    let service_fun = self.service.call(req);
                    Box::pin(async move {
                        let mut result = service_fun.await.map(|e| e.map_into_left_body())?;
                        let _ = result.response_mut().add_cookie(&create_jwt_token_cookie(&new_token));
                        Ok(result)
                    })
                }
            }
        }else{
            let service_fun = self.service.call(req);
            Box::pin(async move {
                let result = service_fun.await.map(|e| e.map_into_left_body())?;
                Ok(result)
            })
        }

    }
}

fn ignore_path(path: &str) -> bool {
    SETTINGS.app_config.jwt_handler_ignore_path.iter().any(|p| path == p)
}
