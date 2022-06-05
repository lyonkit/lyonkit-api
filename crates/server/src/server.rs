use crate::{config::Settings, services::api_services};
use actix_web::{web, App, HttpServer};
use derive_more::Constructor;
use getset::Getters;
use migration::{Migrator, MigratorTrait};
use sea_orm::DatabaseConnection;
use tracing_actix_web::TracingLogger;

#[derive(Clone, Getters, Constructor)]
#[getset(get = "pub")]
pub struct Server {
  server_addr: String,
  database_connection: DatabaseConnection,
}

#[derive(Debug, Clone, Getters)]
#[getset(get = "pub")]
pub struct AppState {
  conn: DatabaseConnection,
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
      settings.server_addr(),
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
    let app_state = AppState {
      conn: self.database_connection.clone(),
    };

    let server_addr = self.server_addr;

    let server = HttpServer::new(move || {
      App::new()
        .wrap(TracingLogger::default())
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
    self.server.await
  }
}
