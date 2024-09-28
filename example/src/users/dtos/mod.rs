pub mod requests;
pub mod responses;
pub mod storage;

use rustium::{
    datastore::idb::{Creatable, Deletable, Patchable},
    surrealdb::sql::Value,
};
use std::collections::BTreeMap;

use crate::users::{
    dtos::{
        requests::{
            CreateAdminRequestDTO, CreateUserRequestDTO, UpdateAdminRequestDTO,
            UpdateUserRequestDTO,
        },
        responses::UserDTO,
        storage::{CreateUserDTO, UpdateUserDTO},
    },
    model::{User, UserType},
};

impl Creatable for CreateUserDTO {}
impl Deletable for UserDTO {}
impl Patchable for UpdateUserDTO {}

impl From<User> for UserDTO {
    fn from(val: User) -> Self {
        UserDTO {
            name: val.name,
            email: val.email,
            user_type: val.user_type,
        }
    }
}

impl From<CreateUserRequestDTO> for CreateUserDTO {
    fn from(val: CreateUserRequestDTO) -> Self {
        let password_hash =
            User::hash_password(val.password.clone()).expect("Should hash password while casting");
        CreateUserDTO {
            name: val.name,
            email: val.email,
            password: password_hash,
            user_type: UserType::Basic,
        }
    }
}

impl From<UpdateUserRequestDTO> for UpdateUserDTO {
    fn from(val: UpdateUserRequestDTO) -> Self {
        let password_hash =
            User::hash_password(val.password.clone()).expect("Should hash password while casting");
        UpdateUserDTO {
            name: val.name,
            email: val.email,
            password: password_hash,
            user_type: UserType::Basic,
        }
    }
}

impl From<CreateAdminRequestDTO> for CreateUserDTO {
    fn from(val: CreateAdminRequestDTO) -> Self {
        let password_hash =
            User::hash_password(val.password.clone()).expect("Should hash password while casting");
        CreateUserDTO {
            name: val.name,
            email: val.email,
            password: password_hash,
            user_type: UserType::Admin,
        }
    }
}

impl From<UpdateAdminRequestDTO> for UpdateUserDTO {
    fn from(val: UpdateAdminRequestDTO) -> Self {
        let password_hash =
            User::hash_password(val.password.clone()).expect("Should hash password while casting");
        UpdateUserDTO {
            name: val.name,
            email: val.email,
            password: password_hash,
            user_type: UserType::Admin,
        }
    }
}

impl From<UserDTO> for Value {
    fn from(val: UserDTO) -> Self {
        BTreeMap::from([
            ("name", val.name.into()),
            ("email".into(), val.email.into()),
            (
                "user_type".into(),
                match val.user_type {
                    UserType::Dev => 0.into(),
                    UserType::Admin => 1.into(),
                    UserType::Agent => 2.into(),
                    UserType::Basic => 3.into(),
                },
            ),
        ])
        .into()
    }
}

impl From<UpdateUserDTO> for Value {
    fn from(val: UpdateUserDTO) -> Self {
        BTreeMap::from([
            ("name", val.name.into()),
            ("email".into(), val.email.into()),
            ("password".into(), val.password.into()),
        ])
        .into()
    }
}

impl From<CreateUserDTO> for Value {
    fn from(val: CreateUserDTO) -> Self {
        BTreeMap::from([
            ("name", val.name.into()),
            ("email".into(), val.email.into()),
            (
                "user_type".into(),
                match val.user_type {
                    UserType::Dev => 0.into(),
                    UserType::Admin => 1.into(),
                    UserType::Agent => 2.into(),
                    UserType::Basic => 3.into(),
                },
            ),
            ("password".into(), val.password.into()),
        ])
        .into()
    }
}
