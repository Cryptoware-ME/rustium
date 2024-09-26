use crate::{prelude::*, settings::RustiumSettings};

// region: Struct

#[derive(Clone)]
pub struct AppContext {
    pub settings: RustiumSettings,
}
// endregion: Struct

// region: Implementation

impl AppContext {
    pub async fn new(settings: RustiumSettings) -> RustiumResult<Self> {
        Ok(AppContext { settings })
    }
}
// endregion: Implementation
