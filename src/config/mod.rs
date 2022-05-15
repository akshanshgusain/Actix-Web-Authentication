pub mod crypto;

use std::sync::Arc;
use std::time::Duration;
use color_eyre::Result;
use serde::Deserialize;
use dotenv::dotenv;
use eyre::WrapErr;
use tracing_subscriber::EnvFilter;
use tracing::{info, instrument};
use sqlx::postgres::PgPool;
use crate::config::crypto::CryptoService;


#[derive(Debug, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: i32,
    pub database_url: String,
    pub secret_key: String,
    pub jwt_secret: String,
}

impl Config {
    #[instrument]
    pub fn from_env() -> Result<Config> {
        dotenv().ok();

        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .init();
        info!("Loading Configurations");
        let mut c = config::Config::new();
        c.merge(config::Environment::default())?;
        c.try_into()
            .context("loading configuration from the environment")
    }

    #[instrument(skip(self))]
    pub async fn dp_pool(&self) -> Result<PgPool> {
        info!("Creating database connection pool.");
        PgPool::builder()
            .connect_timeout(Duration::from_secs(30))s
            .build(&self.database_url)
            .await
            .context("creating database connection pool...")
    }

    pub fn crypto_service(&self) -> CryptoService {
        CryptoService{
            key: Arc::new(self.secret_key.clone())
        }
    }
}