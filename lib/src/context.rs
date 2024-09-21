use di::*;
use std::{
    ops::Deref,
    sync::{Arc, Mutex},
};

use crate::{
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
    services: Arc<Mutex<ServiceCollection>>,
    provider: Option<Arc<ServiceProvider>>,
}
// endregion: Struct

// region: Implementation

impl AppContext {
    pub async fn new(conf: Settings) -> RustiumResult<Self> {
        Ok(AppContext {
            db: Arc::new(SurrealDAL::new(conf.database).await?),
            redis: Arc::new(Mutex::new(RedisDAL::new(conf.redis).await?)),
            rabbit: Arc::new(RabbitDAL::new(conf.rabbit).await?),
            services: Arc::new(Mutex::new(ServiceCollection::new())),
            provider: Option::None,
        })
    }

    pub async fn register_service<T: RustiumService, U>(&self) -> RustiumResult<Self> {
        let this_service = Arc::clone(&self.services);
        let mut this_service = match this_service.lock() {
            Ok(t_service) => t_service,
            Err(_) => {
                return Err(RustiumError::LockError(
                    "Unable to lock service container access".into(),
                ))
            }
        };
        this_service.replace(T::singleton());
        Ok(self.clone())
    }

    pub async fn build_service_providers(&mut self) -> RustiumResult<Self> {
        let this_service = Arc::clone(&self.services);
        let this_service = match this_service.lock() {
            Ok(t_service) => t_service,
            Err(_) => {
                return Err(RustiumError::LockError(
                    "Unable to lock service container access".into(),
                ))
            }
        };
        self.provider = Some(Arc::new(this_service.build_provider()?));
        Ok(self.clone())
    }

    pub fn get_dal(&self) -> RustiumResult<Arc<SurrealDAL>> {
        Ok(Arc::clone(&self.db))
    }

    pub fn get_redis(&self) -> RustiumResult<Arc<Mutex<RedisDAL>>> {
        Ok(Arc::clone(&self.redis))
    }

    pub fn get_rabbit(&self) -> RustiumResult<Arc<RabbitDAL>> {
        Ok(Arc::clone(&self.rabbit))
    }

    pub fn get_service<T: RustiumService>(&self) -> RustiumResult<Ref<T>> {
        match &self.provider {
            Some(provider) => Ok(provider.get_required::<T>()),
            None => Err(RustiumError::ServiceInversionError(String::from(
                "Service of type ",
            ))),
        }
    }
}

// endregion: Implementation
