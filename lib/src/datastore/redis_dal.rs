use redis::{
    Client, Commands, Connection, ConnectionAddr, ConnectionInfo, FromRedisValue,
    RedisConnectionInfo, SetExpiry, SetOptions,
};

use crate::{prelude::*, settings::redis::RedisSettings};
// region: Structs

///
pub struct RedisDAL {
    pub con: Connection,
}
// endregion: Structs

// region: Implementation

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
        Ok(Self { con })
    }

    pub fn get<T: FromRedisValue>(&mut self, key: &str) -> RustiumResult<T> {
        Ok(self.con.get(key)?)
    }

    pub fn set<T: FromRedisValue>(
        &mut self,
        key: &str,
        val: u64,
        exp: SetExpiry,
    ) -> RustiumResult<T> {
        Ok(self.con.set_options(
            key,
            val,
            SetOptions::default().get(true).with_expiration(exp),
        )?)
    }
}
// endregion: Implementation
