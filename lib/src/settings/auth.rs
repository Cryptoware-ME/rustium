use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct AuthSettings {
    pub secret: String,
}
