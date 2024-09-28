use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct LoggerSettings {
    pub level: String,
}
