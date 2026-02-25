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
use interface::IRustiumSettings;
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

#[derive(Debug, Clone, Deserialize, Default)]
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

#[injectable(IRustiumSettings)]
impl RustiumSettings {
    fn new() -> Self {
        let mut this = Self::default();
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current()
                .block_on(this.init())
        })
        .expect("Configuration files should be available and complete");
        this
    }
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
            .add_source(File::with_name(&format!("config/{}.json", run_mode)))
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

    async fn run(&self) -> RustiumResult<()> {
        Ok(())
    }

    fn as_rustium(&self) -> RustiumResult<Option<Box<&dyn RustiumService>>> {
        Ok(Some(Box::new(self)))
    }
}
