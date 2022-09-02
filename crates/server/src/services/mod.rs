mod blok;
mod image;
mod git_json_file;
mod page;
mod post;
mod quote;

use crate::middlewares::s3::S3ProviderMiddlewareFactory;
use crate::services::image::image_service;
use crate::services::git_json_file::{git_json_file_service};
use crate::services::post::post_service;
use crate::services::quote::quote_service;
use crate::{
  middlewares::api_key::ApiKeyMiddlewareFactory,
  services::{blok::blok_service, page::page_service},
};
use actix_web::{
  dev::{ServiceFactory, ServiceRequest, ServiceResponse},
  get, web, HttpResponse, Responder, Scope,
};
use serde_json::json;

#[get("/ping")]
async fn ping() -> impl Responder {
  HttpResponse::Ok().json(json!({
    "message": "Hello LyonKit API !"
  }))
}

pub fn api_services() -> Scope<
  impl ServiceFactory<
    ServiceRequest,
    Response = ServiceResponse,
    Error = actix_web::Error,
    Config = (),
    InitError = (),
  >,
> {
  web::scope("/api")
    .app_data(web::JsonConfig::default().error_handler(|err, _req| {
      actix_web::error::InternalError::from_response(
        "",
        HttpResponse::BadRequest()
          .content_type("application/json")
          .body(format!(r#"{{"code": "JSNER", "message":"{}"}}"#, err)),
      )
      .into()
    }))
    .wrap(ApiKeyMiddlewareFactory::new())
    .wrap(S3ProviderMiddlewareFactory::new())
    .service(ping)
    .service(page_service())
    .service(blok_service())
    .service(image_service())
    .service(post_service())
    .service(quote_service())
    .service(git_json_file_service())
}
