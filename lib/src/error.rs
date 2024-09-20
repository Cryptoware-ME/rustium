//! This is the main application Error type.

use std::convert::Infallible;

// region: Enum

#[derive(Debug)]
pub enum RustiumError {
    IO(std::io::Error),
    PropertyNotFound(String),
    Unexpected(Infallible),
    SurrealError(surrealdb::error::Db),
    StoreFailToCreate(String),
    ModqlOperatorNotSupported(String),
    CreateTableError(String),
    UserNotAllowed(String),
}

// endregion: Enum

// region: From Implementations

impl From<std::io::Error> for RustiumError {
    fn from(val: std::io::Error) -> Self {
        RustiumError::IO(val)
    }
}

impl From<Infallible> for RustiumError {
    fn from(val: Infallible) -> Self {
        RustiumError::Unexpected(val)
    }
}

impl From<surrealdb::error::Db> for RustiumError {
    fn from(val: surrealdb::error::Db) -> Self {
        RustiumError::SurrealError(val)
    }
}
// endregion: From Implementations

// region: Error Boiler

impl std::fmt::Display for RustiumError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> core::result::Result<(), std::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for RustiumError {}
// endregion: Error Boiler
