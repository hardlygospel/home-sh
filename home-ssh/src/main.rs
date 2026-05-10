mod ssh;
mod state;
mod app;

use anyhow::{Context, Result};
use home_core::{config::Config, db};
use tracing_subscriber::{fmt, EnvFilter};

#[tokio::main]
async fn main() -> Result<()> {
    fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("home_ssh=info".parse()?))
        .init();

    let config = Config::from_env().context("Failed to load config")?;

    tracing::info!("Connecting to database...");
    let pool = db::create_pool(&config)
        .await
        .context("Failed to create database pool")?;

    db::run_migrations(&pool)
        .await
        .context("Failed to run migrations")?;

    tracing::info!("Starting SSH server on port {}", config.ssh_port);
    ssh::run_server(pool, config).await?;

    Ok(())
}
