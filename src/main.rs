use clap::Parser;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::info;

use template_please_change::infra::app;
use template_please_change::config::Config;
use template_please_change::infra::database;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::parse();

    let address = SocketAddr::from(([0, 0, 0, 0], config.port));
    let listener = TcpListener::bind(address).await?;
    info!("Server listening on {}", &address);

    let pool = database::create_connection(config.database_url).await?;
    database::run_migrations(pool.clone()).await?;
    info!("Database migrations completed");

    let app = app::create_app(pool, config.allowed_origins)
        .await;
    axum::serve(listener, app).await?;

    Ok(())
}
