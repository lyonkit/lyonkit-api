use crate::services::quote::routes::{create_quote, delete_quote, list_quotes, update_quote};
use actix_web::web::scope;
use actix_web::Scope;

mod routes;
mod models;

pub fn quote_service() -> Scope {
  scope("/quote")
    .service(list_quotes)
    .service(create_quote)
    .service(update_quote)
    .service(delete_quote)
}
