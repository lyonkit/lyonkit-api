#![feature(once_cell)]
#![feature(result_option_inspect)]

mod config;
mod errors;
mod middlewares;
mod server;
mod services;
mod telemetry;

use crate::{config::SETTINGS, server::Server, telemetry::init_tracing};
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
  init_tracing();

  dbg!((*SETTINGS).clone());

  Server::from_settings((*SETTINGS).clone())
    .await
    .run_until_stopped()
    .await?;

  Ok(())
}
