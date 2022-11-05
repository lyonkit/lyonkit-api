use crate::services::quote::routes::{
    create_quote, delete_quote, get_quote, list_quotes, update_quote,
};
use actix_web::{web::scope, Scope};

mod models;
mod routes;

pub fn quote_service() -> Scope {
    scope("/quote")
        .service(list_quotes)
        .service(get_quote)
        .service(create_quote)
        .service(update_quote)
        .service(delete_quote)
}
