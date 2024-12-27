use crate::tools::jwt::{AppJwtToken, JWT_TOKEN_KEY};
use actix::fut::{ready, Ready};
use actix_web::body::EitherBody;
use actix_web::dev::forward_ready;
use actix_web::{dev::{Service, ServiceRequest, ServiceResponse, Transform}, Error, HttpResponse};
use futures_util::future::LocalBoxFuture;

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
        //认证签名
        let verify = req.cookie(JWT_TOKEN_KEY).map(|cookie| {
            let jwt_token = cookie.value();
            AppJwtToken::verify_token_str(jwt_token).ok()
        }).flatten();
        if let None = verify {
            // 认证失败，返回错误响应
            let error_res = HttpResponse::Unauthorized().finish().map_into_right_body();
            return Box::pin(async { Ok(req.into_response(error_res)) });
        }
        let uri = req.path().to_string();
        println!("Hi from start. You requested: {}", uri);
        let service_fun = self.service.call(req);

        Box::pin(async move {
            let result = service_fun.await.map(|e|e.map_into_left_body())?;
            Ok(result)
        })
    }
}

// const ERROR_RESPONSE:HttpResponse = HttpResponse::Unauthorized().finish();

