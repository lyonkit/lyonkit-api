use aws_smithy_async::rt::sleep::default_async_sleep;
use aws_smithy_http::endpoint::Endpoint;
use aws_types::app_name::AppName;
use aws_types::region::Region;
use aws_types::Credentials;
use config::Environment;
pub use config::{Config, ConfigError};
use console::style;
use derive_more::Constructor;
use getset::Getters;
use lazy_static::lazy_static;
use once_cell::sync::Lazy as SyncLazy;
use regex::Regex;
use serde::Deserialize;
use struct_field_names_as_array::FieldNamesAsArray;
use url::Url;

#[derive(Deserialize, Getters, Constructor, Clone, Debug, FieldNamesAsArray)]
#[getset(get = "pub")]
pub struct Settings {
  app_name: String,
  host: String,
  port: String,
  database_url: String,
  telemetry: bool,
  s3: S3Config,
  cors: Vec<String>,
}

#[derive(Deserialize, Getters, Constructor, Clone, Debug, FieldNamesAsArray)]
#[getset(get = "pub")]
pub struct S3Config {
  endpoint: String,
  base_url: String,
  region: String,
  credentials: S3Credentials,
  buckets: S3Buckets,
}

impl From<Settings> for aws_sdk_s3::Config {
  fn from(cfg: Settings) -> Self {
    let s3_cfg = cfg.s3();
    aws_sdk_s3::config::Builder::new()
      .app_name(
        AppName::new(cfg.app_name().clone())
          .expect("Invalid app name given (S3 doesn't accept such app name)"),
      )
      .endpoint_resolver(Endpoint::immutable(
        s3_cfg
          .endpoint()
          .clone()
          .parse()
          .expect("Invalid S3 endpoint provided"),
      ))
      .region(Region::new(s3_cfg.region().clone()))
      .credentials_provider(s3_cfg.credentials().to_sdk_credentials())
      .sleep_impl(default_async_sleep().unwrap())
      .build()
  }
}

#[derive(Deserialize, Getters, Constructor, Clone, Debug)]
#[getset(get = "pub")]
pub struct S3Credentials {
  access_key_id: String,
  secret_access_key: String,
}

impl From<&S3Credentials> for Credentials {
  fn from(creds: &S3Credentials) -> Self {
    Credentials::new(
      creds.access_key_id(),
      creds.secret_access_key(),
      None,
      None,
      "lyonkit_env",
    )
  }
}

impl S3Credentials {
  fn to_sdk_credentials(&self) -> Credentials {
    self.into()
  }
}

#[derive(Deserialize, Getters, Constructor, Clone, Debug)]
#[getset(get = "pub")]
pub struct S3Buckets {
  image: String,
}

impl Settings {
  pub fn from_env() -> Result<Self, ConfigError> {
    let cfg = Config::builder()
      .set_default("app_name", "lyonkit-api")?
      .set_default("port", 8080)?
      .set_default("host", "0.0.0.0")?
      .set_default("telemetry", false)?
      .set_default("s3.buckets.image", "lyonkit-images")?
      .set_default("cors", Vec::new() as Vec<String>)?
      .add_source(
        Environment::default()
          .try_parsing(true)
          .with_list_parse_key("cors")
          .separator("__")
          .list_separator(","),
      )
      .build()
      .unwrap();

    cfg.try_deserialize()
  }

  pub fn server_addr(&self) -> String {
    format!("{host}:{port}", host = self.host, port = self.port)
  }

  #[allow(unused)]
  pub fn database_url_without_db(&self) -> String {
    let mut parsed_url: Url = self
      .database_url()
      .parse()
      .expect("Invalid database url (cannot parse)");

    parsed_url.set_path("");
    parsed_url.to_string()
  }

  #[allow(unused)]
  pub fn database_name(&self) -> String {
    let parsed_url: Url = self
      .database_url()
      .parse()
      .expect("Invalid database url (cannot parse)");

    parsed_url.path().replace('/', "")
  }
}

lazy_static! {
  static ref MISSING_FIELD_RE: Regex = Regex::new("missing field `([A-Za-z0-9_]*)`").unwrap();
}

pub static SETTINGS: SyncLazy<Settings> = SyncLazy::new(|| match Settings::from_env() {
  Ok(settings) => return settings,
  Err(e) => {
    while let Err(ConfigError::Message(msg)) = &Settings::from_env() {
      if let Some(caps) = MISSING_FIELD_RE.captures(msg) {
        if let Some(re_match) = caps.get(1) {
          let field = re_match.as_str();
          let mut env_var = String::from("");

          if Settings::FIELD_NAMES_AS_ARRAY.contains(&field) {
            env_var = field.to_uppercase();
          } else if "credentials" == field {
            env_var =
              String::from("S3__CREDENTIALS__ACCESS_KEY_ID | S3__CREDENTIALS__SECRET_ACCESS_KEY");
            std::env::set_var("S3__CREDENTIALS__ACCESS_KEY_ID", "FOO");
            std::env::set_var("S3__CREDENTIALS__SECRET_ACCESS_KEY", "FOO");
          } else if S3Config::FIELD_NAMES_AS_ARRAY.contains(&field) {
            env_var = format!("S3__{}", field.to_uppercase());
          }

          println!(
            "{}: Missing environnement variable {}",
            style("ERROR").red().bold(),
            style(&env_var).yellow().underlined()
          );

          std::env::set_var(env_var, "FOO");
        }
      }
    }

    panic!("An error occured while parsing config from env : {:?}", e);
  }
});
