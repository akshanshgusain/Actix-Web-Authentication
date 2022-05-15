#[macro_use]
extern crate validator_derive;

use color_eyre::Result;
use crate::config::Config;
use actix_web::{App, HttpServer, middleware::Logger};
use actix_web::web::Data;

mod config;

use tracing::{info};
use crate::config::crypto::CryptoService;
use crate::handlers::app_config;

mod handlers;
mod models;

#[actix_web::main]
async fn main() -> Result<()> {
    let config = Config::from_env()
        .expect("Server Configuration");

    let pool = config.db_pool().await
        .expect("Database configuration");

    let crypto_service: CryptoService = config.crypto_service();

    info!("Starting server at http://{}:{}/", config.host, config.port);
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(Data::new(pool.clone()))
            .app_data(Data::new(crypto_service.clone()))
            .configure(app_config)
    })
        .bind(format!("{}:{}", config.host, config.port))?
        .run()
        .await?;
    Ok({})
}
