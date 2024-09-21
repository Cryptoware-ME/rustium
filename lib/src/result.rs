//! This is the main application Result type.
//! This should be used instead of anyhow::Result.

use crate::error::RustiumError;

/// Custom Result type to use with app-specific errors
/// A replacement for anyhow::
pub type RustiumResult<T> = core::result::Result<T, RustiumError>;
