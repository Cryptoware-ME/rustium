use axum::async_trait;

use crate::prelude::*;

#[async_trait]
pub trait RustiumService: Send + Sync + 'static {
    async fn boot(&self) -> RustiumResult<bool>;
    async fn run(&self) -> RustiumResult<bool>;
}
