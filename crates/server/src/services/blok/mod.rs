use crate::services::blok::routes::{create_blok, get_blok};
use actix_web::{web::scope, Scope};

mod models;
mod routes;

pub fn blok_service() -> Scope {
  scope("/blok").service(get_blok).service(create_blok)
}
