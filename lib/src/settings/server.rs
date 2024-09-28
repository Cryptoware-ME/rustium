use serde::Deserialize;
use std::fmt;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct ServerSettings {
    pub port: u16,
}

impl fmt::Display for ServerSettings {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0.0.0.0:{}", &self.port)
    }
}
