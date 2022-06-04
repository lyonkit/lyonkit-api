use crate::services::page::routes::{
  create_page, delete_page, get_page_with_blok, list_pages, update_page,
};
use actix_web::{web::scope, Scope};

mod models;
mod routes;

pub fn page_service() -> Scope {
  scope("/page")
    .service(list_pages)
    .service(get_page_with_blok)
    .service(create_page)
    .service(update_page)
    .service(delete_page)
}
