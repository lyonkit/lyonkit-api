use ::serde::Deserialize;
use config::Environment;
pub use config::{Config, ConfigError};
use getset::Getters;
use std::lazy::SyncLazy;

#[derive(Deserialize, Getters, Clone)]
#[getset(get = "pub")]
pub struct Settings {
  app_name: String,
  database_url: String,
  telemetry: bool,
  port: String,
  host: String,
}

impl Settings {
  pub fn from_env() -> Result<Self, ConfigError> {
    let cfg = Config::builder()
      .set_default("app_name", "lyonkit-api")?
      .set_default("port", 8080)?
      .set_default("host", "0.0.0.0")?
      .set_default("telemetry", false)?
      .add_source(Environment::default().separator("__").list_separator(","))
      .build()
      .unwrap();

    cfg.try_deserialize()
  }

  pub fn server_addr(&self) -> String {
    format!("{host}:{port}", host = self.host, port = self.port)
  }
}

pub static SETTINGS: SyncLazy<Settings> =
  SyncLazy::new(|| Settings::from_env().expect("Invalid configuration : check for missing env"));
