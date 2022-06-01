use getset::Getters;
use migration::{Migrator, MigratorTrait};
use portpicker::pick_unused_port;
use sea_orm::{ConnectionTrait, DatabaseConnection};
use serde::Serialize;
use server::{
  config::Settings,
  server::Server,
  telemetry::{get_subscriber, init_subscriber},
};
use std::lazy::SyncLazy;
use url::Url;

static TRACING: SyncLazy<()> = SyncLazy::new(|| {
  let default_filter_level = "info".to_string();
  let subscriber_name = "test".to_string();
  match std::env::var("TEST_LOG") {
    Ok(_) => {
      init_subscriber(get_subscriber(
        subscriber_name,
        default_filter_level,
        false,
        std::io::stdout,
      ));
    }
    Err(_) => {
      init_subscriber(get_subscriber(
        subscriber_name,
        default_filter_level,
        false,
        std::io::sink,
      ));
    }
  };
});

#[derive(Getters)]
#[getset(get = "pub")]
pub struct TestApp {
  address: String,
  #[allow(unused)]
  port: u16,
  #[allow(unused)]
  database_connection: DatabaseConnection,
  settings: Settings,
  http_client: reqwest::Client,
}

impl TestApp {
  pub async fn get<S: AsRef<str>>(&self, uri: S) -> reqwest::Response {
    self
      .http_client
      .get(&format!("http://{}/api{}", self.address(), uri.as_ref()))
      .send()
      .await
      .expect("Failed to execute request")
  }

  #[allow(unused)]
  pub async fn post<T: Serialize, S: AsRef<str>>(&self, uri: S, body: T) -> reqwest::Response {
    self
      .http_client
      .post(&format!("http://{}/api{}", self.address(), uri.as_ref()))
      .json(&body)
      .send()
      .await
      .expect("Failed to execute request")
  }

  pub async fn teardown(&self) {
    let conn = sea_orm::Database::connect(self.settings().database_url_without_db())
      .await
      .expect("Failed to connect to database");

    conn
      .execute(sea_orm::Statement::from_string(
        conn.get_database_backend(),
        format!(
          r#"DROP DATABASE IF EXISTS "{db}" WITH (FORCE)"#,
          db = self.settings().database_name()
        ),
      ))
      .await
      .expect("Failed to drop database");
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

  let test_db_name = uuid::Uuid::new_v4().to_string();

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
    port.to_string(),
    database_url,
    false,
  );

  let database_connection = configure_database(&settings).await;

  let server = Server::from_settings(&settings)
    .await
    .build()
    .expect("Failed to start server");

  let address = server.server_addr().clone();

  let _server_process = tokio::spawn(server.run_until_stopped());

  let http_client = reqwest::Client::builder()
    .redirect(reqwest::redirect::Policy::none())
    .cookie_store(true)
    .build()
    .unwrap();

  TestApp {
    http_client,
    address,
    port,
    database_connection,
    settings,
  }
}
