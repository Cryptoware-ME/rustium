use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct ApiSettings {
    pub name: String,
    pub version: u64,
}
