use axum::{async_trait, extract::FromRequestParts, http::request::Parts, RequestPartsExt};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use std::sync::Arc;

use crate::{authentication::token::TokenUser, error::AuthenticateError, prelude::*};

#[async_trait]
impl<T> FromRequestParts<Arc<T>> for TokenUser {
    type Rejection = RustiumError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<T>,
    ) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthenticateError::InvalidToken)?;

        let secret = crate::domain::user::SECRET;
        let token_data =
            token::decode(bearer.token(), secret).map_err(|_| AuthenticateError::InvalidToken)?;

        let _user = match User::get(token_data.claims.user.id.clone(), state).await {
            Ok(user) => user,
            Err(_) => {
                return Err(Error::AuthenticationError(AuthenticateError::InvalidToken));
            }
        };

        Ok(token_data.claims.user)
    }
}
