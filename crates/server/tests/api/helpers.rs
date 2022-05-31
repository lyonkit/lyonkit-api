use getset::Getters;
use sea_orm::DatabaseConnection;
use serde::Serialize;
use server::telemetry::{get_subscriber, init_subscriber};
use std::{env::VarError, lazy::SyncLazy};

static TRACING: SyncLazy<()> = SyncLazy::new(|| {
  let default_filter_level = "info".to_string();
  let subscriber_name = "test".to_string();
  let subscriber = match std::env::var("TEST_LOG") {
    Ok(_) => get_subscriber(
      subscriber_name,
      default_filter_level,
      false,
      std::io::stdout,
    ),
    Err(_) => get_subscriber(subscriber_name, default_filter_level, false, std::io::sink),
  };

  init_subscriber(subscriber);
});

#[derive(Getters)]
#[getset(get = "pub")]
pub struct TestApp {
  address: String,
  port: u16,
  database_connection: DatabaseConnection,
  api_client: reqwest::Client,
}

impl TestApp {
  pub async fn post<T: Serialize>(&self, uri: string, body: T) -> reqwest::Response {
    self
      .api_client
      .post(&format!("{}/api{}", self.address(), uri))
      .json(body)
      .send()
      .await
      .expect("Failed to execute request")
  }
}

pub async fn spawn_app() -> TestApp {
  SyncLazy::force(&TRACING);
}
