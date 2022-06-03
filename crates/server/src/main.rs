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

  let server = Server::from_settings(&*SETTINGS).await.build()?;

  server.run_until_stopped().await?;

  Ok(())
}
