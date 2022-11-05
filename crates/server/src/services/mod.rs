pub mod blok;
pub mod files;
pub mod git_json_file;
pub mod image;
pub mod locale;
pub mod page;
pub mod post;
pub mod quote;

use crate::{
    middlewares::{api_key::ApiKeyMiddlewareFactory, s3::S3ProviderMiddlewareFactory},
    services::{
        blok::blok_service, files::file_service, git_json_file::git_json_file_service,
        image::image_service, locale::locale_service, page::page_service, post::post_service,
        quote::quote_service,
    },
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
        .service(locale_service())
        .service(git_json_file_service())
        .service(file_service())
}
