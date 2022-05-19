use ::serde::Deserialize;
use config::Environment;
pub use config::{Config, ConfigError};
use std::lazy::SyncLazy;

#[derive(Deserialize)]
pub struct Settings {
  pub app_name: String,
  pub database_url: String,
  pub port: String,
  pub host: String,
}

impl Settings {
  pub fn from_env() -> Result<Self, ConfigError> {
    let cfg = Config::builder()
      .set_default("app_name", "lyonkit-api")?
      .set_default("port", 8080)?
      .set_default("host", "0.0.0.0")?
      .add_source(Environment::default().separator(".").list_separator(","))
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
