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

pub trait IRustiumSettings: RustiumService {
    fn get_environment(&self) -> RustiumResult<String>;
    fn get_api(&self) -> RustiumResult<ApiSettings>;
    fn get_server(&self) -> RustiumResult<ServerSettings>;
    fn get_logger(&self) -> RustiumResult<LoggerSettings>;
    fn get_database(&self) -> RustiumResult<DatabaseSettings>;
    fn get_auth(&self) -> RustiumResult<AuthSettings>;
    fn get_redis(&self) -> RustiumResult<RedisSettings>;
    fn get_rabbit(&self) -> RustiumResult<RabbitSettings>;
}

#[derive(Debug, Clone, Deserialize, Default)]
#[injectable(IRustiumSettings)]
pub struct RustiumSettings {
    environment: String,
    api: ApiSettings,
    server: ServerSettings,
    logger: LoggerSettings,
    database: DatabaseSettings,
    auth: AuthSettings,
    redis: RedisSettings,
    rabbit: RabbitSettings,
}

impl IRustiumSettings for RustiumSettings {
    fn get_environment(&self) -> RustiumResult<String> {
        Ok(self.environment.clone())
    }

    fn get_api(&self) -> RustiumResult<ApiSettings> {
        Ok(self.api.clone())
    }

    fn get_server(&self) -> RustiumResult<ServerSettings> {
        Ok(self.server.clone())
    }

    fn get_logger(&self) -> RustiumResult<LoggerSettings> {
        Ok(self.logger.clone())
    }

    fn get_database(&self) -> RustiumResult<DatabaseSettings> {
        Ok(self.database.clone())
    }

    fn get_auth(&self) -> RustiumResult<AuthSettings> {
        Ok(self.auth.clone())
    }

    fn get_redis(&self) -> RustiumResult<RedisSettings> {
        Ok(self.redis.clone())
    }

    fn get_rabbit(&self) -> RustiumResult<RabbitSettings> {
        Ok(self.rabbit.clone())
    }
}

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
