use crate::services::locale::routes::{get_locales, update_locale};
use actix_web::{web::scope, Scope};

mod models;
mod repository;
mod routes;

pub fn locale_service() -> Scope {
    scope("/locale").service(get_locales).service(update_locale)
}
