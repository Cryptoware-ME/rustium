use amqprs::connection::{Connection, OpenConnectionArguments};

use crate::{prelude::*, settings::rabbit::RabbitSettings};

// region: Structs

pub struct RabbitDAL {
    pub con: Connection,
}
// endregion: Structs

// region: Implementation

impl RabbitDAL {
    pub async fn new(conf: RabbitSettings) -> RustiumResult<Self> {
        let con = Connection::open(&OpenConnectionArguments::new(
            &conf.server,
            conf.port.into(),
            &conf.username,
            &conf.password,
        ))
        .await?;

        Ok(Self { con })
    }
}
// endregion: Implementation
