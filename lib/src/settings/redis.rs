use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct RedisSettings {
    pub server: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub dbidx: i64,
}
