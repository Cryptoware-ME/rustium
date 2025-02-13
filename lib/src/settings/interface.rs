use axum::async_trait;

use crate::{
    prelude::*,
    service::RustiumService,
    settings::{
        ApiSettings, AuthSettings, DatabaseSettings, LoggerSettings, RabbitSettings, RedisSettings,
        ServerSettings,
    },
};

#[async_trait]
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
