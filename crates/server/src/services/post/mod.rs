use crate::services::post::routes::{
  create_post, delete_post, get_post, get_post_by_slug, list_posts, update_post,
};
use actix_web::{web::scope, Scope};

mod models;
mod routes;

pub fn post_service() -> Scope {
  scope("/post")
    .service(list_posts)
    .service(get_post)
    .service(get_post_by_slug)
    .service(create_post)
    .service(update_post)
    .service(delete_post)
}
