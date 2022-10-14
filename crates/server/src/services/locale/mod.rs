use crate::services::locale::routes::{get_locales, update_locale};
use actix_web::web::scope;
use actix_web::Scope;

mod models;
mod routes;

pub fn locale_service() -> Scope {
    scope("/locale").service(get_locales).service(update_locale)
}
