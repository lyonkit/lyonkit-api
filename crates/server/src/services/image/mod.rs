use crate::services::image::routes::upload_image;
use actix_web::web::scope;
use actix_web::Scope;

mod models;
mod routes;

pub fn image_service() -> Scope {
  scope("/image").service(upload_image)
}
