use axum::async_trait;
use di::injectable;
use redis::{
    Client, Commands, Connection, ConnectionAddr, ConnectionInfo, RedisConnectionInfo, SetExpiry,
    SetOptions, Value,
};

use crate::{datastore::ikv_dal::IKvDal, prelude::*, settings::redis::RedisSettings};
// region: Structs

#[injectable(IKvDal)]
pub struct RedisDAL {
    pub con: Option<Connection>,
}
// endregion: Structs

// region: Implementation

impl Default for RedisDAL {
    fn default() -> Self {
        Self { con: Option::None }
    }
}

impl RedisDAL {
    pub async fn new(conf: RedisSettings) -> RustiumResult<Self> {
        let client = Client::open(ConnectionInfo {
            addr: ConnectionAddr::Tcp(conf.server.into(), conf.port.into()),
            redis: RedisConnectionInfo {
                db: conf.dbidx.into(),
                protocol: redis::ProtocolVersion::RESP3,
                username: Some(conf.username.into()),
                password: Some(conf.password.into()),
            },
        })?;
        let con = client.get_connection()?;
        Ok(Self { con: Some(con) })
    }
}

#[async_trait]
impl IKvDal for RedisDAL {
    async fn get(&mut self, key: &str) -> RustiumResult<Value> {
        Ok(self
            .con
            .as_mut()
            .expect("Redis service not initialized")
            .get(key)?)
    }

    async fn set(&mut self, key: &str, val: u64, exp: SetExpiry) -> RustiumResult<Value> {
        Ok(self
            .con
            .as_mut()
            .expect("Redis service not initialized")
            .set_options(
                key,
                val,
                SetOptions::default().get(true).with_expiration(exp),
            )?)
    }
}
// endregion: Implementation
