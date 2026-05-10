use anyhow::{Context, Result};
use deadpool_postgres::{Config as PgConfig, ManagerConfig, Pool, RecyclingMethod, Runtime};
use tokio_postgres::NoTls;

use crate::config::Config;

pub async fn create_pool(config: &Config) -> Result<Pool> {
    let mut pg_config = PgConfig::new();
    pg_config.host = Some(config.db_host.clone());
    pg_config.port = Some(config.db_port);
    pg_config.user = Some(config.db_user.clone());
    pg_config.password = Some(config.db_password.clone());
    pg_config.dbname = Some(config.db_name.clone());
    pg_config.manager = Some(ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    });

    let pool = pg_config
        .create_pool(Some(Runtime::Tokio1), NoTls)
        .context("Failed to create database pool")?;

    Ok(pool)
}

pub async fn run_migrations(pool: &Pool) -> Result<()> {
    let client = pool.get().await.context("Failed to get DB client")?;

    let migration_sql = include_str!("../migrations/001_initial.sql");
    client
        .batch_execute(migration_sql)
        .await
        .context("Failed to run migrations")?;

    tracing::info!("Database migrations completed successfully");
    Ok(())
}
