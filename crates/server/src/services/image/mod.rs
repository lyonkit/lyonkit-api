use crate::services::image::routes::{delete_image, list_images, upload_image};
use actix_web::{web::scope, Scope};

mod models;
mod routes;

pub fn image_service() -> Scope {
    scope("/image")
        .service(list_images)
        .service(upload_image)
        .service(delete_image)
}
