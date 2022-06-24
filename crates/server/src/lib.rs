use crate::config::SETTINGS;
use crate::server::Server;
use crate::telemetry::init_tracing;
use std::io;

pub mod config;
pub mod errors;
pub mod middlewares;
pub mod server;
pub mod services;
pub mod telemetry;

pub async fn main() -> io::Result<()> {
  openssl_probe::init_ssl_cert_env_vars();
  init_tracing();

  let server = Server::from_settings(&*SETTINGS)
    .await
    .migrate()
    .await
    .build()?;

  server.run_until_stopped().await.ok();

  opentelemetry::global::shutdown_tracer_provider();

  Ok(())
}
