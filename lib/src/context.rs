use shaku::Module;
use std::sync::Arc;

use crate::prelude::*;

// region: Struct

#[derive(Clone)]
pub struct AppContext<M: Module> {
    pub module: Arc<M>,
}
// endregion: Struct

// region: Implementation

impl<M: Module> AppContext<M> {
    pub async fn new(module: M) -> RustiumResult<Self> {
        Ok(AppContext {
            module: Arc::new(module),
        })
    }
}
// endregion: Implementation
