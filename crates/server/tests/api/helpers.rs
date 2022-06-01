use async_trait::async_trait;
use getset::Getters;
use migration::{Migrator, MigratorTrait};
use portpicker::pick_unused_port;
use reqwest::Method;
use sea_orm::{ActiveModelTrait, ActiveValue, ConnectionTrait, DatabaseConnection};
use serde::Serialize;
use server::{
  config::Settings,
  server::Server,
  telemetry::{get_subscriber, init_subscriber},
};
use std::lazy::SyncLazy;
use test_context::AsyncTestContext;
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

#[derive(Getters, Clone)]
#[getset(get = "pub")]
pub struct TestApp {
  address: String,
  #[allow(unused)]
  port: u16,
  #[allow(unused)]
  database_connection: DatabaseConnection,
  settings: Settings,
  http_client: reqwest::Client,
  active_api_key: Option<String>,
}

impl TestApp {
  pub async fn req<U: AsRef<str>, T: Serialize>(
    &self,
    method: Method,
    uri: U,
    body: Option<T>,
  ) -> reqwest::Response {
    let mut request = self
      .http_client
      .request(
        method,
        &format!("http://{}/api{}", self.address(), uri.as_ref()),
      )
      .header("Content-Type", "application/json");

    if let Some(api_key) = &self.active_api_key {
      request = request.header("X-Api-Key", api_key);
    }

    if let Some(payload) = &body {
      request = request.json(payload);
    }

    request.send().await.expect("Failed to execute request")
  }

  pub async fn get<S: AsRef<str>>(&self, uri: S) -> reqwest::Response {
    self.req(Method::GET, uri, None as Option<()>).await
  }

  pub async fn post<T: Serialize, S: AsRef<str>>(&self, uri: S, body: T) -> reqwest::Response {
    self.req(Method::POST, uri, Some(body)).await
  }

  pub async fn create_api_key(
    &mut self,
    namespace: &str,
    read_only: bool,
  ) -> entity::api_key::Model {
    let api_key = entity::api_key::ActiveModel {
      namespace: ActiveValue::set(namespace.to_owned()),
      read_only: ActiveValue::set(read_only),
      ..Default::default()
    };
    let api_key: entity::api_key::Model = api_key
      .insert(self.database_connection())
      .await
      .expect("Failed to create API key");
    self.active_api_key = Some(api_key.key.to_string());
    api_key
  }

  pub async fn terminate(&self) {
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
    active_api_key: None,
  }
}

#[async_trait]
impl AsyncTestContext for TestApp {
  async fn setup() -> Self {
    spawn_app().await
  }

  async fn teardown(self) {
    self.terminate().await
  }
}
