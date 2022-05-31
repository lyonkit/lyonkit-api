use crate::{config::Settings, services::api_services};
use actix_web::{web, App, HttpServer};
use derive_more::Constructor;
use getset::Getters;
use sea_orm::DatabaseConnection;
use tracing_actix_web::{DefaultRootSpanBuilder, RootSpanBuilder, TracingLogger};

#[derive(Clone, Getters, Constructor)]
#[getset(get = "pub")]
pub struct Server<RootSpan: RootSpanBuilder> {
  server_addr: String,
  database_connection: DatabaseConnection,
  logger: TracingLogger<RootSpan>,
}

#[derive(Debug, Clone, Getters)]
#[getset(get = "pub")]
pub struct AppState {
  conn: DatabaseConnection,
}

impl Server<DefaultRootSpanBuilder> {
  pub async fn from_settings(settings: Settings) -> Self {
    Self::new(
      settings.server_addr(),
      sea_orm::Database::connect(settings.database_url())
        .await
        .expect(
          "Failed to connect to the database, please ensure the given env DATABASE_URL is valid !",
        ),
      TracingLogger::default(),
    )
  }
}

impl<RootSpan: RootSpanBuilder + Send + 'static> Server<RootSpan> {
  pub async fn run_until_stopped(self) -> std::io::Result<()> {
    let app_state = AppState {
      conn: self.database_connection,
    };

    HttpServer::new(move || {
      App::new()
        .wrap(self.logger.clone())
        .app_data(web::Data::new(app_state.clone()))
        .service(api_services())
    })
    .bind(self.server_addr)?
    .run()
    .await
  }
}
