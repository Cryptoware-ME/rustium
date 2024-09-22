use std::{
    any::type_name,
    sync::{Arc, Mutex, RwLock},
};

use crate::{
    authentication::auth_service::AuthService,
    datastore::{rabbit_dal::RabbitDAL, redis_dal::RedisDAL, surreal_dal::SurrealDAL},
    prelude::*,
    service::RustiumService,
    settings::Settings,
};

// region: Struct

#[derive(Clone)]
pub struct AppContext {
    db: Arc<SurrealDAL>,
    redis: Arc<Mutex<RedisDAL>>,
    rabbit: Arc<RabbitDAL>,
}
// endregion: Struct

// region: Implementation

impl AppContext {
    pub async fn new(conf: Settings) -> RustiumResult<Self> {
        Ok(AppContext {
            db: Arc::new(SurrealDAL::new(conf.database).await?),
            redis: Arc::new(Mutex::new(RedisDAL::new(conf.redis).await?)),
            rabbit: Arc::new(RabbitDAL::new(conf.rabbit).await?),
        })
    }

    pub async fn register_service<T: RustiumService>(&self) -> RustiumResult<Self> {}

    pub async fn build_service_providers(&mut self) -> RustiumResult<Self> {}

    pub fn get_dal(&self) -> RustiumResult<Arc<SurrealDAL>> {
        Ok(Arc::clone(&self.db))
    }

    pub fn get_redis(&self) -> RustiumResult<Arc<Mutex<RedisDAL>>> {
        Ok(Arc::clone(&self.redis))
    }

    pub fn get_rabbit(&self) -> RustiumResult<Arc<RabbitDAL>> {
        Ok(Arc::clone(&self.rabbit))
    }

    pub fn get_service<T: 'static + ?Sized>(&self) -> RustiumResult<Arc<T>> {}
}

// endregion: Implementation
