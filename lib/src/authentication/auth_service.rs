use crate::authentication::token::Claims;

pub trait AuthService {
    fn get_claim_user(&self, token_claim: Claims) -> String;
}
