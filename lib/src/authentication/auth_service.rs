use crate::{authentication::token::Claims, service::RustiumService};

pub trait AuthService: RustiumService {
    fn get_claim_user(&self, token_claim: Claims) -> String;
}
