use rustium::{
    axum::Json, di_axum::Inject, http::StatusCode, prelude::*, response::RustiumResponse,
};

use crate::users::{
    dtos::{requests::CreateUserRequestDTO, responses::UserDTO},
    service::IUserService,
};

pub async fn create_user(
    Inject(user_service): Inject<dyn IUserService>,
    Json(body): Json<CreateUserRequestDTO>,
) -> RustiumResult<RustiumResponse<UserDTO>> {
    if user_service
        .check_exists(body.email.clone(), body.name.clone())
        .await
    {
        return Err(RustiumError::UserAlreadyExists(
            "A user with this email or username already exists".into(),
        ));
    }

    let user = user_service.create(body.into()).await?;
    let res = UserDTO::from(user_service.get(user.0).await?);

    let res = RustiumResponse::new()
        .data(res)
        .status_code(StatusCode::CREATED)
        .build();

    Ok(res)
}
