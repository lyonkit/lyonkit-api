use serde::Deserialize;
use config::Environment;
pub use config::{Config, ConfigError};
use derive_more::Constructor;
use getset::Getters;
use std::lazy::SyncLazy;
use url::Url;

#[derive(Deserialize, Getters, Constructor, Clone, Debug)]
#[getset(get = "pub")]
pub struct Settings {
  app_name: String,
  host: String,
  port: String,
  database_url: String,
  telemetry: bool,
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

  #[allow(unused)]
  pub fn database_url_without_db(&self) -> String {
    let mut parsed_url =
      Url::parse(self.database_url().as_str()).expect("Invalid database url (cannot parse)");

    parsed_url.set_path("");
    parsed_url.to_string()
  }

  #[allow(unused)]
  pub fn database_name(&self) -> String {
    let parsed_url =
      Url::parse(self.database_url().as_str()).expect("Invalid database url (cannot parse)");

    parsed_url.path().replace("/", "")
  }
}

pub static SETTINGS: SyncLazy<Settings> =
  SyncLazy::new(|| Settings::from_env().expect("Invalid configuration : check for missing env"));
