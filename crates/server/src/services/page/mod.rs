use crate::services::page::create::create_page;
use actix_web::{web::scope, Scope};

mod create;

pub fn page_service() -> Scope {
  scope("/page").service(create_page)
}
