use axum::async_trait;

use crate::{
    authentication::{auth_user::AuthUser, token::Claims},
    prelude::*,
    service::RustiumService,
};

/// EXAMPLE
/// ```
/// struct User {}

/// impl AuthUser for User {
///     fn get_id(&self) -> String {
///         todo!()
///     }

///     fn get_username(&self) -> String {
///         todo!()
///     }

///     fn is_admin(&self) -> bool {
///         todo!()
///     }
/// }

/// #[injectable(IAuthService)]
/// struct ExampleAuthService;

/// impl RustiumService for ExampleAuthService {}

/// #[async_trait]
/// impl IAuthService for ExampleAuthService {
///     async fn get_claim_user(&self, token_claim: Claims) -> RustiumResult<Box<dyn AuthUser>> {
///         todo!()
///     }
/// }
/// ```
#[async_trait]
pub trait IAuthService: RustiumService {
    async fn get_claim_user(&self, token_claim: Claims) -> RustiumResult<Box<dyn AuthUser>>;
}
