use actix_web::{web::scope, Scope};

use crate::services::files::routes::{create_file, list_files};

pub mod models;
pub mod repository;
pub mod routes;

pub fn file_service() -> Scope {
    scope("/file").service(create_file).service(list_files)
}
