use serde::{Deserialize, Serialize};

use crate::users::model::UserType;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserDTO {
    pub name: String,
    pub email: String,
    pub user_type: UserType,
}
