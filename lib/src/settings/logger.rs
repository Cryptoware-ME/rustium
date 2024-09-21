use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct LoggerSettings {
    pub level: String,
}
