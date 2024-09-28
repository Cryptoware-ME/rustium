pub mod api;
pub mod auth;
pub mod database;
pub mod interface;
pub mod logger;
pub mod rabbit;
pub mod redis;
pub mod server;

use axum::async_trait;
use config::{Config, Environment, File};
use di::injectable;
use serde::Deserialize;
use std::env;

use crate::{
    service::RustiumService,
    settings::{
        api::ApiSettings, auth::AuthSettings, database::DatabaseSettings, logger::LoggerSettings,
        rabbit::RabbitSettings, redis::RedisSettings, server::ServerSettings,
    },
    RustiumResult,
};

pub trait IRustiumSettings {}

#[derive(Debug, Clone, Deserialize, Default)]
#[injectable(IRustiumSettings)]
pub struct RustiumSettings {
    pub environment: String,
    pub api: ApiSettings,
    pub server: ServerSettings,
    pub logger: LoggerSettings,
    pub database: DatabaseSettings,
    pub auth: AuthSettings,
    pub redis: RedisSettings,
    pub rabbit: RabbitSettings,
}

impl IRustiumSettings for RustiumSettings {}

#[async_trait]
impl RustiumService for RustiumSettings {
    async fn init(&mut self) -> RustiumResult<()> {
        let run_mode = env::var("RUSTIUM_ENV").unwrap_or_else(|_| "development".into());

        let builder = Config::builder()
            .add_source(File::with_name(&format!("config/{run_mode}")))
            .add_source(Environment::default().separator("__"));

        let temp: Self = builder.build()?.try_deserialize()?;

        self.environment = temp.environment;
        self.api = temp.api;
        self.server = temp.server;
        self.logger = temp.logger;
        self.database = temp.database;
        self.auth = temp.auth;
        self.redis = temp.redis;
        self.rabbit = temp.rabbit;

        Ok(())
    }

    async fn run(&mut self) -> RustiumResult<()> {
        Ok(())
    }

    fn as_rustium(&mut self) -> RustiumResult<Option<Box<&mut dyn RustiumService>>> {
        Ok(Some(Box::new(self)))
    }
}
