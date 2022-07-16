use crate::services::blok::routes::{create_blok, delete_blok, get_blok, patch_blok, update_blok};
use actix_web::{web::scope, Scope};

pub(crate) mod models;
mod routes;

pub fn blok_service() -> Scope {
  scope("/blok")
    .service(get_blok)
    .service(create_blok)
    .service(update_blok)
    .service(patch_blok)
    .service(delete_blok)
}
