mod models;
mod routes;
mod services;

use crate::services::git_json_file::routes::{get_git_json_file, update_git_json_file};
use actix_web::{web::scope, Scope};

pub fn git_json_file_service() -> Scope {
    scope("/git/json-file")
        .service(get_git_json_file)
        .service(update_git_json_file)
}
