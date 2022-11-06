use actix_web::{web::scope, Scope};

use crate::services::files::routes::{create_file, delete_file, list_files, update_file};

pub mod models;
pub mod repository;
pub mod routes;

pub fn file_service() -> Scope {
    scope("/file")
        .service(create_file)
        .service(list_files)
        .service(update_file)
        .service(delete_file)
}
