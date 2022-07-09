use crate::services::post::routes::{create_post, delete_post, get_post, list_posts, update_post};
use actix_web::{web::scope, Scope};

mod models;
mod routes;

pub fn post_service() -> Scope {
  scope("/post")
    .service(list_posts)
    .service(get_post)
    .service(create_post)
    .service(update_post)
    .service(delete_post)
}
