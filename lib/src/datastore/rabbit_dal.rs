use amqprs::connection::{Connection, OpenConnectionArguments};
use di::injectable;

use crate::{datastore::imq_dal::IMqDal, prelude::*, settings::rabbit::RabbitSettings};

// region: Structs

#[injectable(IMqDal)]
pub struct RabbitDAL {
    pub con: Option<Connection>,
}
// endregion: Structs

// region: Implementation

impl IMqDal for RabbitDAL {}
impl Default for RabbitDAL {
    fn default() -> Self {
        Self { con: Option::None }
    }
}

impl RabbitDAL {
    pub async fn new(conf: RabbitSettings) -> RustiumResult<Self> {
        let con = Connection::open(&OpenConnectionArguments::new(
            &conf.server,
            conf.port.into(),
            &conf.username,
            &conf.password,
        ))
        .await?;

        Ok(Self { con: Some(con) })
    }
}
// endregion: Implementation
