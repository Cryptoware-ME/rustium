use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct ApiSettings {
    pub name: String,
    pub version: u64,
}
