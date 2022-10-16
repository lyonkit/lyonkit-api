use crate::pick_unused_port::{pick_unused_port, release_port};
use crate::utils;
use async_trait::async_trait;
use aws_sdk_s3::Client;
use getset::{Getters, Setters};
use migration::{Migrator, MigratorTrait};
use once_cell::sync::Lazy as SyncLazy;
use reqwest::Method;
use sea_orm::{ActiveModelTrait, ActiveValue, ConnectionTrait, DatabaseConnection};
use serde::Serialize;
use serde_json::json;
use server::config::{LogFormat, S3Buckets, S3Config, S3Credentials};
use server::{
    config::Settings,
    server::Server,
    telemetry::{get_subscriber, init_subscriber},
};
use std::env;
use test_context::AsyncTestContext;
use url::Url;
use uuid::Uuid;

static TRACING: SyncLazy<()> = SyncLazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();
    let test_log = env::var("TEST_LOG").ok().and_then(|test_log| {
        if test_log == "true" || test_log == "1" {
            Some(test_log)
        } else {
            None
        }
    });
    match test_log {
        Some(_) => {
            init_subscriber(get_subscriber(
                subscriber_name,
                default_filter_level,
                false,
                std::io::stdout,
            ));
        }
        None => {
            init_subscriber(get_subscriber(
                subscriber_name,
                default_filter_level,
                false,
                std::io::sink,
            ));
        }
    };
});

#[derive(Getters, Setters, Clone)]
#[getset(get = "pub")]
pub struct TestApp {
    address: String,
    #[allow(unused)]
    port: u16,
    #[allow(unused)]
    database_connection: DatabaseConnection,
    #[allow(unused)]
    s3_client: aws_sdk_s3::Client,
    settings: Settings,
    http_client: reqwest::Client,
    #[getset(set = "pub")]
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
    pub async fn req_multipart<U: AsRef<str>>(
        &self,
        method: Method,
        uri: U,
        body: reqwest::multipart::Form,
    ) -> reqwest::Response {
        let mut request = self.http_client.request(
            method,
            &format!("http://{}/api{}", self.address(), uri.as_ref()),
        );

        if let Some(api_key) = &self.active_api_key {
            request = request.header("X-Api-Key", api_key);
        }

        request = request.multipart(body.into());

        request.send().await.expect("Failed to execute request")
    }

    pub async fn get<S: AsRef<str>>(&self, uri: S) -> reqwest::Response {
        self.req(Method::GET, uri, None as Option<()>).await
    }

    pub async fn post<T: Serialize, S: AsRef<str>>(&self, uri: S, body: T) -> reqwest::Response {
        self.req(Method::POST, uri, Some(body)).await
    }

    pub async fn post_multipart<S: AsRef<str>>(
        &self,
        uri: S,
        form: reqwest::multipart::Form,
    ) -> reqwest::Response {
        self.req_multipart(Method::POST, uri, form).await
    }

    pub async fn put<T: Serialize, S: AsRef<str>>(&self, uri: S, body: T) -> reqwest::Response {
        self.req(Method::PUT, uri, Some(body)).await
    }

    pub async fn patch<T: Serialize, S: AsRef<str>>(&self, uri: S, body: T) -> reqwest::Response {
        self.req(Method::PATCH, uri, Some(body)).await
    }

    pub async fn delete<S: AsRef<str>>(&self, uri: S) -> reqwest::Response {
        self.req(Method::DELETE, uri, None as Option<()>).await
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

        conn.execute(sea_orm::Statement::from_string(
            conn.get_database_backend(),
            format!(
                r#"DROP DATABASE IF EXISTS "{db}" WITH (FORCE)"#,
                db = self.settings().database_name()
            ),
        ))
        .await
        .expect("Failed to drop database");

        utils::wipe_bucket(&self.s3_client, self.settings().s3().buckets().image()).await;

        release_port(self.port);
    }
}

async fn configure_database(settings: &Settings) -> DatabaseConnection {
    // Create database
    let conn = sea_orm::Database::connect(settings.database_url_without_db())
        .await
        .expect("Failed to connect to database");

    conn.execute(sea_orm::Statement::from_string(
        conn.get_database_backend(),
        format!(
            r#"DROP DATABASE IF EXISTS "{db}""#,
            db = settings.database_name()
        ),
    ))
    .await
    .expect("Failed to drop database");

    conn.execute(sea_orm::Statement::from_string(
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

async fn configure_s3(settings: &Settings) -> Client {
    let client = Client::from_conf(settings.clone().into());
    let bucket: &String = settings.s3().buckets().image();

    client
        .create_bucket()
        .bucket(bucket)
        .grant_read("*")
        .grant_read_acp("*")
        .send()
        .await
        .ok();

    client
        .put_public_access_block()
        .bucket(bucket)
        .send()
        .await
        .ok();

    client
        .put_bucket_policy()
        .bucket(bucket)
        .policy(
            json!({
              "Version":"2012-10-17",
              "Statement":[
                {
                  "Sid":"PublicRead",
                  "Effect":"Allow",
                  "Principal": "*",
                  "Action":["s3:GetObject"],
                  "Resource":[format!("arn:aws:s3:::{bucket}/*")]
                }
              ]
            })
            .to_string(),
        )
        .send()
        .await
        .ok();

    client
}

pub async fn spawn_app() -> TestApp {
    SyncLazy::force(&TRACING);

    let test_db_name = Uuid::new_v4().to_string();

    let database_url = {
        let mut url: Url = env::var("DATABASE_URL")
            .expect("No database url specified")
            .parse()
            .expect("Invalid database url (cannot parse)");
        url.set_path(&*format!("/{}", test_db_name));
        url.to_string()
    };

    let port = pick_unused_port();
    let s3_bucket = format!("test{}", Uuid::new_v4().to_string().replace('-', ""));
    let settings = Settings::new(
        String::from("test"),
        String::from("0.0.0.0"),
        port.to_string(),
        database_url,
        false,
        S3Config::new(
            env::var("S3__ENDPOINT").expect("No S3 endpoint specified"),
            env::var("S3__BASE_URL").expect("No S3 base url specified"),
            env::var("S3__REGION").expect("No S3 region specified"),
            S3Credentials::new(
                env::var("S3__CREDENTIALS__ACCESS_KEY_ID")
                    .expect("No S3 credentials access key specified"),
                env::var("S3__CREDENTIALS__SECRET_ACCESS_KEY")
                    .expect("No S3 credentials secret key specified"),
            ),
            S3Buckets::new(s3_bucket),
        ),
        Vec::new(),
        LogFormat::Json,
    );

    let database_connection = configure_database(&settings).await;
    let s3_client = configure_s3(&settings).await;

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
        s3_client,
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
