use sea_orm_migration::prelude::*;

#[tokio::main]
async fn main() {
    openssl_probe::init_ssl_cert_env_vars();
    cli::run_cli(migration::Migrator).await;
}
