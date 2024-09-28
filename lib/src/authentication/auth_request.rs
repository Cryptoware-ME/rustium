use axum::{async_trait, extract::FromRequestParts, http::request::Parts, RequestPartsExt};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use di_axum::Inject;
use std::sync::Arc;

use crate::{
    authentication::{
        auth_service::IAuthService,
        auth_user::AuthUser,
        token::{decode_auth_token, TokenUser},
    },
    error::AuthenticateError,
    prelude::*,
    settings::{auth::AuthSettings, IRustiumSettings},
};

#[async_trait]
impl FromRequestParts<Arc<()>> for TokenUser {
    type Rejection = RustiumError;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &Arc<()>,
    ) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthenticateError::InvalidToken)?;

        let Inject(auth_service) =
            parts
                .extract::<Inject<dyn IAuthService>>()
                .await
                .map_err(|_| {
                    RustiumError::NotFound("Authentication service was not initialized".into())
                })?;

        let Inject(settings) = parts
            .extract::<Inject<dyn IRustiumSettings>>()
            .await
            .map_err(|_| RustiumError::NotFound("Settings service was not initialized".into()))?;

        let auth_settings: AuthSettings = settings.get_auth()?;

        let token_data = decode_auth_token(bearer.token(), &auth_settings.secret)
            .map_err(|_| AuthenticateError::InvalidToken)?;

        let user: Box<dyn AuthUser> = auth_service.get_claim_user(token_data.claims).await?;

        Ok(user.into())
    }
}
