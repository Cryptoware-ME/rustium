use axum::async_trait;
use di::*;

use crate::prelude::*;

#[async_trait]
pub trait RustiumService: Send + Sync + Injectable + 'static {
    async fn run(&self) -> RustiumResult<bool>;
}
