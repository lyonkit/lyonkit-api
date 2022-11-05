use crate::{errors::ApiError, server::AppState};
use actix_web::{
    dev::{forward_ready, Payload, Service, ServiceRequest, ServiceResponse, Transform},
    web, Error as ActixError, FromRequest, HttpMessage, HttpRequest,
};
use entity::api_key;
use futures::{future::LocalBoxFuture, FutureExt};
use getset::Getters;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use std::{
    future::{ready, Ready},
    rc::Rc,
};
use tracing::{error, info_span, warn, Instrument};
use uuid::Uuid;

#[derive(Getters, Clone)]
#[getset(get = "pub")]
pub struct ApiKey {
    namespace: String,
    read_only: bool,
}

impl From<api_key::Model> for ApiKey {
    fn from(model: api_key::Model) -> Self {
        Self {
            namespace: model.namespace,
            read_only: model.read_only,
        }
    }
}

#[derive(Clone)]
pub struct MaybeApiKey(Option<ApiKey>);

impl From<Option<api_key::Model>> for MaybeApiKey {
    fn from(model: Option<api_key::Model>) -> Self {
        MaybeApiKey(model.map(|v| v.into()))
    }
}

fn no_api_key_error(req: &HttpRequest) -> ActixError {
    if req.headers().get("x-api-key").is_none() {
        ApiError::ApiKeyNotProvided.into()
    } else {
        ApiError::ApiKeyInvalid.into()
    }
}

impl FromRequest for ApiKey {
    type Error = ActixError;
    type Future = Ready<Result<ApiKey, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let extensions = req.extensions();
        let api_key = extensions.get::<MaybeApiKey>();

        let result = match api_key.cloned().and_then(|key| key.0) {
            Some(v) => Ok(v),
            None => Err(no_api_key_error(req)),
        };

        ready(result)
    }
}

#[derive(Getters, Clone)]
#[getset(get = "pub")]
pub struct WriteApiKey {
    namespace: String,
}

impl FromRequest for WriteApiKey {
    type Error = ActixError;
    type Future = Ready<Result<WriteApiKey, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let extensions = req.extensions();
        let api_key = extensions.get::<MaybeApiKey>();

        let result = match api_key.cloned().and_then(|key| key.0) {
            Some(v) => {
                if v.read_only {
                    Err(ApiError::ApiKeyReadOnly.into())
                } else {
                    Ok(WriteApiKey {
                        namespace: v.namespace,
                    })
                }
            }
            None => Err(no_api_key_error(req)),
        };

        ready(result)
    }
}

impl From<api_key::Model> for WriteApiKey {
    fn from(model: api_key::Model) -> Self {
        Self {
            namespace: model.namespace,
        }
    }
}

#[derive(Default)]
pub struct ApiKeyMiddlewareFactory;

impl ApiKeyMiddlewareFactory {
    pub fn new() -> Self {
        ApiKeyMiddlewareFactory {}
    }
}

impl<S, B> Transform<S, ServiceRequest> for ApiKeyMiddlewareFactory
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = ActixError> + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = ActixError;
    type Transform = ApiKeyMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ApiKeyMiddleware {
            service: Rc::new(service),
        }))
    }
}
pub struct ApiKeyMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for ApiKeyMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = ActixError> + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = ActixError;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let srv = self.service.clone();

        async move {
            let app_data = req
                .app_data::<web::Data<AppState>>()
                .expect("App state is not defined");

            let maybe_api_key = req.headers().get("x-api-key").and_then(|v| v.to_str().ok());
            let mut api_entity: Option<api_key::Model> = None;

            if let Some(api_key) = maybe_api_key {
                let api_key_uuid = Uuid::parse_str(api_key);

                if let Ok(uuid) = api_key_uuid {
                    let api_key_entiy = api_key::Entity::find()
                        .filter(api_key::Column::Key.eq(uuid))
                        .one(app_data.conn())
                        .await
                        .map_err(|db_err| {
                            error!(
                                error_message = format!("{:?}", db_err).as_str(),
                                "An error occured while querying api key"
                            );
                            db_err
                        })
                        .ok()
                        .flatten();

                    api_entity = api_key_entiy.clone();

                    req.extensions_mut()
                        .insert::<MaybeApiKey>(api_key_entiy.into());
                } else {
                    warn!(api_key, "Api key invalid UUID");
                }
            }

            let res = srv
                .call(req)
                .instrument(
                    info_span!(
                      "WITH_API_KEY_MIDDLEWARE",
                      api_key.namespace = ?api_entity.as_ref().map(|v| v.namespace.clone()),
                      api_key.read_only = ?api_entity.as_ref().map(|v| v.read_only)
                    )
                    .or_current(),
                )
                .await?;
            Ok(res)
        }
        .boxed_local()
    }
}
