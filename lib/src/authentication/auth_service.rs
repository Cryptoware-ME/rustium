use shaku::Interface;

use crate::{
    authentication::{auth_user::AuthUser, token::Claims},
    prelude::*,
};

pub trait AuthService: Interface {
    fn get_claim_user(&self, token_claim: Claims) -> RustiumResult<Box<dyn AuthUser>>;
}
