use serde::{Deserialize, Serialize};

use crate::users::model::UserType;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserDTO {
    pub name: String,
    pub email: String,
    pub user_type: UserType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticateResponse {
    pub access_token: String,
    pub user: UserDTO,
}
