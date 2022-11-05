use crate::config::SETTINGS;
use actix_web::{
    dev::{forward_ready, Payload, Service, ServiceRequest, ServiceResponse, Transform},
    Error as ActixError, FromRequest, HttpMessage, HttpRequest,
};
use derive_more::Constructor;
use futures::{future::LocalBoxFuture, FutureExt};
use getset::Getters;
use std::{
    future::{ready, Ready},
    rc::Rc,
    sync::Arc,
};

#[derive(Constructor)]
pub struct S3ProviderMiddlewareFactory;

impl<S, B> Transform<S, ServiceRequest> for S3ProviderMiddlewareFactory
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = ActixError> + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = ActixError;
    type Transform = S3ProviderMiddleware<S>;
    type InitError = ();
    type Future = LocalBoxFuture<'static, Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        async move {
            let client = aws_sdk_s3::Client::from_conf((*SETTINGS).clone().into());
            Ok(S3ProviderMiddleware {
                client: Arc::new(client),
                service: Rc::new(service),
            })
        }
        .boxed_local()
    }
}

pub type S3ClientExt = Arc<aws_sdk_s3::Client>;

#[derive(Getters)]
#[getset(get = "pub")]
pub struct S3ProviderMiddleware<S> {
    client: S3ClientExt,
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for S3ProviderMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = ActixError> + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = ActixError;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();
        let client = self.client.clone();
        async move {
            req.extensions_mut().insert::<S3ClientExt>(client);
            let res = service.call(req).await?;
            Ok(res)
        }
        .boxed_local()
    }
}

pub struct S3ClientProvider(S3ClientExt);

impl FromRequest for S3ClientProvider {
    type Error = ActixError;
    type Future = Ready<Result<S3ClientProvider, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let ext = req.extensions();
        let s3_ext = ext.get::<S3ClientExt>().unwrap();

        ready(Ok(S3ClientProvider(s3_ext.clone())))
    }
}

impl S3ClientProvider {
    pub fn provide(self) -> S3ClientExt {
        self.0
    }
}
