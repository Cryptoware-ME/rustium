use di::ServiceCollection;

use crate::{
    datastore::{rabbit_dal::RabbitDAL, redis_dal::RedisDAL, surreal_dal::SurrealDAL},
    prelude::*,
};
use std::sync::{Arc, Mutex};
// region: Struct

#[derive(Clone)]
pub struct AppContext {
    db: Arc<SurrealDAL>,
    redis: Arc<Mutex<RedisDAL>>,
    rabbit: Arc<RabbitDAL>,
    services: ServiceCollection,
}
// endregion: Struct

// region: Implementation

impl AppContext {
    pub async fn new() -> Result<Self> {}

    pub async fn register_service() -> Result<Self> {}

    pub fn get_dal(&self) -> Arc<SurrealDAL> {}

    pub fn get_redis(&self) -> Arc<Mutex<RedisDAL>> {}

    pub fn get_rabbit(&self) -> Arc<RabbitDAL> {}
}

// endregion: Implementation
