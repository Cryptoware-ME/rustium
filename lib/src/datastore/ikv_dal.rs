use axum::async_trait;
use redis::{SetExpiry, Value};

use crate::prelude::*;

#[async_trait]
pub trait IKvDal {
    async fn get(&mut self, key: &str) -> RustiumResult<Value>;
    async fn set(&mut self, key: &str, val: u64, exp: SetExpiry) -> RustiumResult<Value>;
}
