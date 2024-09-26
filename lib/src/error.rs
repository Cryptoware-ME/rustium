//! This is the main application Error type.

use argon2::Error as ArgonError;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use di::ValidationError;
use redis::RedisError;
use serde_json::json;
use std::convert::Infallible;
use std::num::{ParseFloatError, ParseIntError};
use std::string::FromUtf8Error;
use tokio::task::JoinError;

// region: Enum

#[derive(thiserror::Error, Debug)]
#[error("...")]
pub enum RustiumError {
    #[error("Server error occured")]
    IO(std::io::Error),
    #[error("Resource not found")]
    NotFound(String),
    #[error("Property not found")]
    PropertyNotFound(String),
    #[error("Unexpected error")]
    Unexpected(Infallible),
    #[error("DB engine error")]
    SurrealError(surrealdb::Error),
    #[error("DB operation error")]
    SurrealDBError(surrealdb::error::Db),
    #[error("DB create failed")]
    StoreFailToCreate(String),
    #[error("Query operator not supported")]
    ModqlOperatorNotSupported(String),
    #[error("Error creating table record")]
    CreateTableError(String),
    #[error("Missing or invalid permissions")]
    UserNotAllowed(String),
    #[error("Unresolved error")]
    Unresolved(String),
    #[error("Failed to parse ID")]
    ParseObjectID(String),
    #[error("Authentication error")]
    AuthenticationError(AuthenticateError),
    #[error("Bad request")]
    BadRequest(BadRequest),
    #[error("Tokio task join error")]
    RunSyncTask(JoinError),
    #[error("Argon2 error")]
    HashPassword(ArgonError),
    #[error("Message queue error")]
    AMPQError(amqprs::error::Error),
    #[error("Float parse error")]
    FloatParse(ParseFloatError),
    #[error("Int parse error")]
    IntParse(ParseIntError),
    #[error("User already exists")]
    UserAlreadyExists(String),
    #[error("Invalid signature")]
    InvalidSignature(String),
    #[error("Invalid signer")]
    InvalidSigner(String),
    #[error("UTF8 convert error")]
    ToUTF8Error(FromUtf8Error),
    #[error("Redis error")]
    RedisErr(RedisError),
    #[error("Mutex lock error")]
    LockError(String),
    #[error("An error occured")]
    CustomError(String),
    #[error("A JWT error occured")]
    JWTError(jsonwebtoken::errors::Error),
    #[error("Service is not ready or not loaded")]
    ServiceInversionError(ValidationError),
    // #[error("...")]
    // CustomError(ErrorType),
}

#[derive(thiserror::Error, Debug)]
#[error("...")]
pub enum AuthenticateError {
    #[error("Wrong authentication credentials")]
    WrongCredentials,
    #[error("Failed to create authentication token")]
    TokenCreation,
    #[error("Invalid authentication credentials")]
    InvalidToken,
    #[error("User doesn't have permission to access this resource")]
    UnauthorizedRequest,
    #[error("User is locked")]
    Locked,
}

#[derive(thiserror::Error, Debug)]
#[error("Bad Request")]
pub struct BadRequest {}
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

impl From<surrealdb::Error> for RustiumError {
    fn from(val: surrealdb::Error) -> Self {
        RustiumError::SurrealError(val)
    }
}

impl From<surrealdb::error::Db> for RustiumError {
    fn from(val: surrealdb::error::Db) -> Self {
        RustiumError::SurrealDBError(val)
    }
}

impl From<jsonwebtoken::errors::Error> for RustiumError {
    fn from(val: jsonwebtoken::errors::Error) -> Self {
        RustiumError::JWTError(val)
    }
}

impl From<amqprs::error::Error> for RustiumError {
    fn from(val: amqprs::error::Error) -> Self {
        RustiumError::AMPQError(val)
    }
}

impl From<RedisError> for RustiumError {
    fn from(val: RedisError) -> Self {
        RustiumError::RedisErr(val)
    }
}

impl From<AuthenticateError> for RustiumError {
    fn from(val: AuthenticateError) -> Self {
        RustiumError::AuthenticationError(val)
    }
}

impl From<ValidationError> for RustiumError {
    fn from(val: ValidationError) -> Self {
        RustiumError::ServiceInversionError(val)
    }
}
// endregion: From Implementations

// region: Error status code & into response
impl RustiumError {
    fn get_codes(&self) -> (StatusCode, u16) {
        match *self {
            // 4XX Errors
            RustiumError::ParseObjectID(_) => (StatusCode::BAD_REQUEST, 40001),
            RustiumError::BadRequest(_) => (StatusCode::BAD_REQUEST, 40002),
            RustiumError::NotFound(_) => (StatusCode::NOT_FOUND, 40003),
            RustiumError::AuthenticationError(AuthenticateError::WrongCredentials) => {
                (StatusCode::UNAUTHORIZED, 40004)
            }
            RustiumError::AuthenticationError(AuthenticateError::InvalidToken) => {
                (StatusCode::UNAUTHORIZED, 40005)
            }
            RustiumError::AuthenticationError(AuthenticateError::Locked) => {
                (StatusCode::LOCKED, 40006)
            }
            RustiumError::AuthenticationError(AuthenticateError::UnauthorizedRequest) => {
                (StatusCode::FORBIDDEN, 40007)
            }

            // 5XX Errors
            RustiumError::AuthenticationError(AuthenticateError::TokenCreation) => {
                (StatusCode::INTERNAL_SERVER_ERROR, 5001)
            }
            RustiumError::RunSyncTask(_) => (StatusCode::INTERNAL_SERVER_ERROR, 5005),
            RustiumError::HashPassword(_) => (StatusCode::INTERNAL_SERVER_ERROR, 5006),
            _ => (StatusCode::BAD_REQUEST, 400),
        }
    }

    pub fn bad_request() -> Self {
        RustiumError::BadRequest(BadRequest {})
    }

    pub fn not_found(resource_name: &str) -> Self {
        RustiumError::NotFound(resource_name.into())
    }

    pub fn unauthorized() -> Self {
        RustiumError::AuthenticationError(AuthenticateError::UnauthorizedRequest)
    }
}

impl IntoResponse for RustiumError {
    fn into_response(self) -> Response {
        let (status_code, code) = self.get_codes();
        let message = self.to_string();
        let body = Json(json!({ "code": code, "message": message }));

        (status_code, body).into_response()
    }
}
// endregion: Error status code & into response
