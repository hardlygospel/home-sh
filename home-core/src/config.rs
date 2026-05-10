use anyhow::{Context, Result};
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub ssh_port: u16,
    pub db_host: String,
    pub db_user: String,
    pub db_password: String,
    pub db_name: String,
    pub db_port: u16,
    pub icecast_url: String,
    pub liquidsoap_url: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        Ok(Config {
            ssh_port: env::var("HOME_SSH_PORT")
                .unwrap_or_else(|_| "2222".to_string())
                .parse()
                .context("HOME_SSH_PORT must be a valid port number")?,
            db_host: env::var("HOME_DB_HOST").unwrap_or_else(|_| "localhost".to_string()),
            db_user: env::var("HOME_DB_USER").unwrap_or_else(|_| "postgres".to_string()),
            db_password: env::var("HOME_DB_PASSWORD").unwrap_or_else(|_| "postgres".to_string()),
            db_name: env::var("HOME_DB_NAME").unwrap_or_else(|_| "homedb".to_string()),
            db_port: env::var("HOME_DB_PORT")
                .unwrap_or_else(|_| "5432".to_string())
                .parse()
                .context("HOME_DB_PORT must be a valid port number")?,
            icecast_url: env::var("HOME_ICECAST_URL")
                .unwrap_or_else(|_| "http://localhost:8000".to_string()),
            liquidsoap_url: env::var("HOME_LIQUIDSOAP_URL")
                .unwrap_or_else(|_| "http://localhost:8001".to_string()),
        })
    }

    pub fn db_connection_string(&self) -> String {
        format!(
            "host={} port={} user={} password={} dbname={}",
            self.db_host, self.db_port, self.db_user, self.db_password, self.db_name
        )
    }
}
