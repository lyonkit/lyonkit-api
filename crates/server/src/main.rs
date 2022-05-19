#![feature(once_cell)]
#![feature(result_option_inspect)]

mod config;
mod errors;
mod middlewares;
mod services;
mod telemetry;

use crate::{config::SETTINGS, telemetry::init_telemetry};
use actix_web::{web, App, HttpServer};
use sea_orm::DatabaseConnection;
use services::api_services;
use std::io;
use tracing_actix_web::TracingLogger;

#[derive(Debug, Clone)]
pub struct AppState {
  conn: DatabaseConnection,
}

#[actix_web::main]
async fn main() -> io::Result<()> {
  init_telemetry();

  let conn = sea_orm::Database::connect(&(*SETTINGS).database_url)
    .await
    .unwrap();
  let server_addr = (*SETTINGS).server_addr();
  let app_state = AppState { conn };

  HttpServer::new(move || {
    App::new()
      .wrap(TracingLogger::default())
      .app_data(web::Data::new(app_state.clone()))
      .service(api_services())
  })
  .bind(server_addr)?
  .run()
  .await?;

  Ok(())
}
