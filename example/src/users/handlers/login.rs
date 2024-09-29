use rustium::{
    authentication::token, axum::Json, di_axum::Inject, error::AuthenticateError, prelude::*,
    settings::IRustiumSettings,
};

use crate::users::{
    dtos::{
        requests::AuthorizeBody,
        responses::{AuthenticateResponse, UserDTO},
    },
    service::IUserService,
};

pub async fn authenticate_user(
    Inject(user_service): Inject<dyn IUserService>,
    Inject(settings_service): Inject<dyn IRustiumSettings>,
    Json(body): Json<AuthorizeBody>,
) -> RustiumResult<Json<AuthenticateResponse>> {
    let email = &body.email;
    let password = &body.password;

    if email.is_empty() {
        return Err(RustiumError::bad_request());
    }

    if password.is_empty() {
        return Err(RustiumError::bad_request());
    }

    let user = user_service.get_by_email(email.clone()).await?;

    if !user_service
        .is_password_match(user.clone(), password)
        .await?
    {
        return Err(RustiumError::AuthenticationError(
            AuthenticateError::WrongCredentials,
        ));
    }

    let auth_settings = settings_service
        .get_auth()
        .expect("Setting service should be loaded");

    let token = token::create(user.clone(), &auth_settings.secret)
        .map_err(|_| RustiumError::AuthenticationError(AuthenticateError::TokenCreation))?;

    let res = AuthenticateResponse {
        access_token: token,
        user: UserDTO::from(user),
    };

    Ok(Json(res))
}
