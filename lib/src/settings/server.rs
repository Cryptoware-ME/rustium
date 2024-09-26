use serde::Deserialize;
use std::fmt;

#[derive(Debug, Clone, Deserialize)]
pub struct ServerSettings {
    pub port: u16,
    pub secret: String,
}

impl fmt::Display for ServerSettings {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0.0.0.0:{}", &self.port)
    }
}
