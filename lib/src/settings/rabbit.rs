use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct RabbitSettings {
    pub server: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}
