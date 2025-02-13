use di::{injectable, Ref};

use rustium::{
    authentication::{auth_user::AuthUser, service::IAuthService, token::Claims},
    axum::async_trait,
    prelude::*,
    service::RustiumService,
};

use crate::users::service::IUserService;

#[injectable(IAuthService)]
pub struct AuthService {
    user_service: Ref<dyn IUserService>,
}

#[async_trait]
impl RustiumService for AuthService {
    fn as_rustium(&self) -> RustiumResult<Option<Box<&dyn RustiumService>>> {
        Ok(Some(Box::new(self)))
    }

    async fn init(&mut self) -> RustiumResult<()> {
        Ok(())
    }

    async fn run(&self) -> RustiumResult<()> {
        Ok(())
    }
}

#[async_trait]
impl IAuthService for AuthService {
    async fn get_claim_user(&self, token_claim: Claims) -> RustiumResult<Box<dyn AuthUser>> {
        Ok(Box::new(
            self.user_service.get(token_claim.user.id.clone()).await?,
        ))
    }
}
