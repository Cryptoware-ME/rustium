use rustium::{
    axum::extract::Path, datastore::idb::IRustiumDb, di::*, di_axum::*, http::StatusCode,
    prelude::*, response::RustiumResponse, settings::interface::IRustiumSettings,
};

use crate::users::{
    dtos::{requests::CreateUserRequestDTO, responses::UserDTO, storage::CreateUserDTO},
    service::IUserService,
};

pub async fn get_user(
    Inject(user_service): Inject<dyn IUserService>,
    Path(id): Path<String>,
) -> RustiumResult<RustiumResponse<UserDTO>> {
    Ok(RustiumResponse::new()
        .data(UserDTO::from(user_service.get(id).await?))
        .status_code(StatusCode::OK)
        .build())
}

pub async fn health_check(
    Inject(settings): Inject<dyn IRustiumSettings>,
    // Inject(db): Inject<dyn IRustiumDb>,
) -> RustiumResult<RustiumResponse<String>> {
    // let cu: CreateUserDTO = (CreateUserRequestDTO {
    //     name: "Jad".to_string(),
    //     email: "jad.jabbour@cryptoware.me".to_string(),
    //     password: "112233".to_string(),
    // })
    // .into();
    // let _ = db.exec_create("users", cu.try_into()?).await?;
    Ok(RustiumResponse::new()
        .data(String::from(settings.get_environment()?))
        .status_code(StatusCode::OK)
        .build())
}
