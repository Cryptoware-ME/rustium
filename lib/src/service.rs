use axum::async_trait;

use crate::RustiumResult;

pub trait RustiumThreadSafe: Send + Sync {}

#[async_trait]
pub trait RustiumService: Send + Sync + 'static {
    fn as_rustium(&mut self) -> RustiumResult<Option<Box<&mut dyn RustiumService>>>;
    async fn init(&mut self) -> RustiumResult<()>;
    async fn run(&mut self) -> RustiumResult<()>;
}
