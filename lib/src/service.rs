pub trait RustiumThreadSafe: Send + Sync {}
pub trait RustiumService: Send + Sync + 'static {}
