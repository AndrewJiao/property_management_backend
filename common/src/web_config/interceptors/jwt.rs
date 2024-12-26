use actix::fut::{ready, Ready};
use actix_web::dev::forward_ready;
use actix_web::{dev::{Service, ServiceRequest, ServiceResponse, Transform}, Error};
use futures_util::future::LocalBoxFuture;
use log::info;

pub struct JWTMiddleware;

impl<S, B> Transform<S, ServiceRequest> for JWTMiddleware
where
    S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
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
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;
    forward_ready!(service);
    fn call(&self, req: ServiceRequest) -> Self::Future {
        let uri = req.path().to_string();
        println!("Hi from start. You requested: {}", uri);
        let service_fun = self.service.call(req);
        Box::pin(async move {
            //认证签名
            let str = format!(" JWTMiddleware  for url {uri}");
            info!("{}", str);
            let result = service_fun.await?;
            info!("JWTMiddleware  for url {uri}  end");
            Ok(result)
        })
    }
}

