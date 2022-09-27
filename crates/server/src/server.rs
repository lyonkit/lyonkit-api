use crate::{config::Settings, services::api_services};
use actix_cors::Cors;
use actix_web::http::header::{HeaderName, CONTENT_DISPOSITION, CONTENT_TYPE, ORIGIN};
use actix_web::http::Method;
use actix_web::{web, App, HttpServer};
use derive_more::Constructor;
use getset::Getters;
use migration::{Migrator, MigratorTrait};
use sea_orm::DatabaseConnection;
use tracing_actix_web::TracingLogger;

#[derive(Clone, Getters, Constructor)]
#[getset(get = "pub")]
pub struct Server {
  settings: Settings,
  database_connection: DatabaseConnection,
}

#[derive(Debug, Clone, Getters)]
#[getset(get = "pub")]
pub struct AppState {
  conn: DatabaseConnection,
  settings: Settings,
}

#[derive(Getters)]
#[getset(get = "pub")]
pub struct ActiveServer {
  server: actix_web::dev::Server,
  #[allow(unused)]
  server_addr: String,
}

impl Server {
  pub async fn from_settings(settings: &Settings) -> Self {
    Self::new(
      settings.clone(),
      sea_orm::Database::connect(settings.database_url())
        .await
        .expect(
          "Failed to connect to the database, please ensure the given env DATABASE_URL is valid !",
        ),
    )
  }

  pub async fn migrate(self) -> Self {
    Migrator::up(&self.database_connection, None)
      .await
      .expect("Failed to apply migrations");

    self
  }

  pub fn build(self) -> std::io::Result<ActiveServer> {
    let settings = self.settings();

    let app_state = AppState {
      conn: self.database_connection.clone(),
      settings: settings.clone(),
    };

    let server_addr = settings.server_addr();

    let cors_endpoints = settings.cors().clone();

    let server = HttpServer::new(move || {
      let mut cors = Cors::default()
        .allowed_methods(&[
          Method::GET,
          Method::POST,
          Method::PUT,
          Method::PATCH,
          Method::DELETE,
        ])
        .allowed_headers(&[
          HeaderName::from_static("x-api-key"),
          ORIGIN,
          CONTENT_TYPE,
          CONTENT_DISPOSITION,
        ]);

      for endpoint in &cors_endpoints {
        cors = cors.allowed_origin(endpoint.as_ref());
      }

      App::new()
        .wrap(TracingLogger::default())
        .wrap(cors)
        .app_data(web::Data::new(app_state.clone()))
        .service(api_services())
    })
    .bind(server_addr.clone())?
    .run();

    Ok(ActiveServer {
      server,
      server_addr,
    })
  }
}

impl ActiveServer {
  pub async fn run_until_stopped(self) -> std::io::Result<()> {
    let res = self.server.await;
    res
  }
}
