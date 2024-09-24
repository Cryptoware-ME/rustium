use axum::{async_trait, extract::FromRequestParts, http::request::Parts, RequestPartsExt};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use shaku::{HasComponent, Module};
use std::{env, sync::Arc};

use crate::{
    authentication::{
        auth_service::AuthService,
        auth_user::AuthUser,
        token::{decode_auth_token, TokenUser},
    },
    context::AppContext,
    error::AuthenticateError,
    prelude::*,
};

#[async_trait]
impl<M: Module + HasComponent<dyn AuthService>> FromRequestParts<Arc<AppContext<M>>> for TokenUser {
    type Rejection = RustiumError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<AppContext<M>>,
    ) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthenticateError::InvalidToken)?;

        let secret = env::var("RUSTIUM_AUTH_SECRET").unwrap_or_else(|_| "secret".into());

        let token_data = decode_auth_token(bearer.token(), &secret)
            .map_err(|_| AuthenticateError::InvalidToken)?;

        let module = Arc::clone(&state.module);
        let service: Arc<dyn AuthService> = module.resolve();
        let token_claims = token_data.clone();
        let user: Box<dyn AuthUser> = service.get_claim_user(token_claims.claims)?;

        Ok(user.into())
    }
}
