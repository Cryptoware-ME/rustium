use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct AuthSettings {
    pub secret: String,
}
