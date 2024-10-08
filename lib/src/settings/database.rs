use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct DatabaseSettings {
    pub uri: String,
    pub username: String,
    pub password: String,
    pub namespace: String,
    pub dbname: String,
}
