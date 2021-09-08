use std::net::SocketAddr;

use axum_jwt_example::config;
use clap::Clap;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    use config::db::DbPool;

    if dotenv::dotenv().is_err() {
        tracing::warn!("missing .env file in current working directory");
    };
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .pretty()
        .init();

    let pg_pool = sqlx::PgPool::retreive().await;
    let config = config::env::ServerConfig::parse();
    let addr = SocketAddr::from((config.host, config.port));
    tracing::debug!("listening on {}", addr);
    let server =
        axum::Server::bind(&addr).serve(axum_jwt_example::app(pg_pool).into_make_service());

    if let Err(err) = server.await {
        tracing::error!("server error: {:?}", err);
    }
}