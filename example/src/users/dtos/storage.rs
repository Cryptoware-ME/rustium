use rustium::{
    modql::filter::{FilterNodes, OpValsInt64, OpValsString},
    serde::{Deserialize, Serialize},
};

use crate::users::model::UserType;

#[derive(FilterNodes, Debug)]
pub struct UserFilter {
    pub id: Option<OpValsString>,
    pub name: Option<OpValsString>,
    pub email: Option<OpValsString>,
    pub user_type: Option<OpValsInt64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateUserDTO {
    pub name: String,
    pub email: String,
    pub password: String,
    pub user_type: UserType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateUserDTO {
    pub name: String,
    pub email: String,
    pub password: String,
    pub user_type: UserType,
}
