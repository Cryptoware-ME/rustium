//! Crate Prelude

pub use crate::error::RustiumError;
pub use crate::result::RustiumResult;

/// Generic Wrapper tuple struct for newtype pattern
/// for external type to type From/TryFrom implementations
pub struct Wrap<T>(pub T);

/// Shorthand (for convenience)
pub use std::format as f;
