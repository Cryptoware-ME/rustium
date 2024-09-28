use rustium::{
    axum::extract::Path, di_axum::Inject, http::StatusCode, prelude::*, response::RustiumResponse,
};

use crate::users::{dtos::responses::UserDTO, service::IUserService};

pub async fn get_user(
    Inject(user_service): Inject<dyn IUserService>,
    Path(id): Path<String>,
) -> RustiumResult<RustiumResponse<UserDTO>> {
    Ok(RustiumResponse::new()
        .data(UserDTO::from(user_service.get(id).await?))
        .status_code(StatusCode::OK)
        .build())
}
