#![feature(once_cell)]
#![feature(result_option_inspect)]

mod config;
mod errors;
mod middlewares;
mod server;
mod services;
mod telemetry;

use crate::{config::SETTINGS, server::Server, telemetry::init_telemetry};
use std::io;
use tracing_actix_web::TracingLogger;

#[tokio::main]
async fn main() -> io::Result<()> {
  init_telemetry();

  Server::new(
    (*SETTINGS).server_addr(),
    sea_orm::Database::connect(&(*SETTINGS).database_url)
      .await
      .expect(
        "Failed to connect to the database, please ensure the given env DATABASE_URL is valid !",
      ),
    TracingLogger::default(),
  )
  .run_until_stopped()
  .await?;

  Ok(())
}
