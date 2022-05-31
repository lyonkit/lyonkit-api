use getset::Getters;
use migration::{Migrator, MigratorTrait};
use opentelemetry::trace::SpanKind::Server;
use portpicker::pick_unused_port;
use sea_orm::{ColumnType::Uuid, ConnectionTrait, DatabaseConnection};
use serde::Serialize;
use server::{
  config::{Settings, SETTINGS},
  server::Server,
  telemetry::{get_subscriber, init_subscriber},
};
use std::{env::VarError, lazy::SyncLazy};
use url::Url;

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
  http_client: reqwest::Client,
}

impl TestApp {
  pub async fn post<T: Serialize>(&self, uri: string, body: T) -> reqwest::Response {
    self
      .http_client
      .post(&format!("{}/api{}", self.address(), uri))
      .json(body)
      .send()
      .await
      .expect("Failed to execute request")
  }
}

async fn configure_database(settings: &Settings) -> DatabaseConnection {
  // Create database
  let conn = sea_orm::Database::connect(settings.database_url_without_db())
    .await
    .expect("Failed to connect to database");

  conn
    .execute(sea_orm::Statement::from_string(
      conn.get_database_backend(),
      format!(
        r#"DROP DATABASE IF EXISTS "{db}""#,
        db = settings.database_name()
      ),
    ))
    .await
    .expect("Failed to drop database");

  conn
    .execute(sea_orm::Statement::from_string(
      conn.get_database_backend(),
      format!(r#"CREATE DATABASE "{db}""#, db = settings.database_name()),
    ))
    .await
    .expect("Failed to create database");

  // Migrate database
  let db_conn = sea_orm::Database::connect(settings.database_url())
    .await
    .expect("Failed to connect to database");
  Migrator::up(&db_conn, None)
    .await
    .expect("Failed to apply migrations");
  db_conn
}

pub async fn spawn_app() -> TestApp {
  SyncLazy::force(&TRACING);

  let test_db_name = "__lyonkit_api_test";

  let database_url = {
    let url = std::env::var("DATABASE_URL").expect("No database url specified");
    let mut parsed = Url::parse(&url).expect("Invalid database url (cannot parse)");
    parsed.set_path(&*format!("/{}", test_db_name));
    parsed.to_string()
  };

  let port = pick_unused_port().expect("No available port");
  let settings = Settings::new(
    String::from("test"),
    String::from("0.0.0.0"),
    port,
    database_url,
    false,
  );

  let database_connection = configure_database(&settings).await;

  let server = Server::from_settings(settings).await;

  let _server_process = tokio::spawn(server.run_until_stopped());

  let http_client = reqwest::Client::builder()
    .redirect(reqwest::redirect::Policy::none())
    .cookie_store(true)
    .build()
    .unwrap();

  TestApp {
    http_client,
    address: server.server_addr().clone(),
    port,
    database_connection,
  }
}
