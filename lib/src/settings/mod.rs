use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::{env, fmt};

#[derive(Debug, Clone, Deserialize)]
pub struct App {
    pub name: String,
    pub version: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Server {
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Logger {
    pub level: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Database {
    pub uri: String,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Auth {
    pub secret: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub environment: String,
    pub server: Server,
    pub logger: Logger,
    pub database: Database,
    pub auth: Auth,
}

impl Settings {
    pub fn new(config_fille_path: &str) -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let builder = Config::builder()
            .add_source(File::with_name(&format!("{config_fille_path}/{run_mode}")))
            .add_source(Environment::default().separator("__"));

        builder.build()?.try_deserialize()
    }
}

impl fmt::Display for Server {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0.0.0.0:{}", &self.port)
    }
}
