use crate::{
    datastore::{rabbit_dal::RabbitDAL, redis_dal::RedisDAL, surreal_dal::SurrealDAL},
    prelude::*,
};
use std::sync::{Arc, Mutex};
// region: Struct

#[derive(Clone)]
pub struct AppContext {
    pub db: Arc<SurrealDAL>,
    pub redis: Arc<Mutex<RedisDAL>>,
    pub rabbit: Arc<RabbitDAL>,
}
// endregion: Struct

// region: Implementation

impl Database {
    pub async fn new() -> Result<Self> {
        Ok(Database(
            Arc::new(SurrealDAL::new("rocksdb://sneakerjardb", "sneakerjar", "db").await?),
            Arc::new(Mutex::new(
                RedisDAL::new("127.0.0.1", 6379, "default", "qH2N3nkvaxKMJWd").await?,
            )),
            Arc::new(RabbitDAL::new("157.245.26.87", 5672, "master", "g6O4KhG33VCX7Ro").await?),
        ))
    }

    pub fn get_dal(&self) -> Arc<SurrealDAL> {
        self.0.clone()
    }

    pub fn get_redis(&self) -> Arc<Mutex<RedisDAL>> {
        Arc::clone(&self.1)
    }

    pub fn get_rabbit(&self) -> Arc<RabbitDAL> {
        self.2.clone()
    }
}

// endregion: Implementation
