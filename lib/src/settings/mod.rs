pub mod api;
pub mod auth;
pub mod database;
pub mod logger;
pub mod rabbit;
pub mod redis;
pub mod server;

use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::env;

use crate::settings::{
    api::ApiSettings, auth::AuthSettings, database::DatabaseSettings, logger::LoggerSettings,
    rabbit::RabbitSettings, redis::RedisSettings, server::ServerSettings,
};

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub environment: String,
    pub api: ApiSettings,
    pub server: ServerSettings,
    pub logger: LoggerSettings,
    pub database: DatabaseSettings,
    pub auth: AuthSettings,
    pub redis: RedisSettings,
    pub rabbit: RabbitSettings,
}

impl Settings {
    pub fn new(config_fille_path: &str) -> Result<Self, ConfigError> {
        let run_mode = env::var("RUSTIUM_ENV").unwrap_or_else(|_| "development".into());

        let builder = Config::builder()
            .add_source(File::with_name(&format!("{config_fille_path}/{run_mode}")))
            .add_source(Environment::default().separator("__"));

        builder.build()?.try_deserialize()
    }
}
