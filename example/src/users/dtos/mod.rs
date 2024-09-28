pub mod requests;
pub mod responses;
pub mod storage;

use rustium::{
    authentication::hash_password,
    datastore::idb::{Creatable, Deletable, Patchable},
    prelude::*,
    surrealdb::sql::Object,
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
            hash_password(val.password.clone()).expect("Should hash password while casting");
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
            hash_password(val.password.clone()).expect("Should hash password while casting");
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
            hash_password(val.password.clone()).expect("Should hash password while casting");
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
            hash_password(val.password.clone()).expect("Should hash password while casting");
        UpdateUserDTO {
            name: val.name,
            email: val.email,
            password: password_hash,
            user_type: UserType::Admin,
        }
    }
}

impl From<UserDTO> for Object {
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

impl TryFrom<UpdateUserDTO> for Object {
    type Error = RustiumError;
    fn try_from(val: UpdateUserDTO) -> RustiumResult<Self> {
        Ok(BTreeMap::from([
            ("name", val.name.into()),
            ("email".into(), val.email.into()),
            ("password".into(), val.password.into()),
        ])
        .into())
    }
}

impl TryFrom<CreateUserDTO> for Object {
    type Error = RustiumError;
    fn try_from(val: CreateUserDTO) -> RustiumResult<Self> {
        Ok(BTreeMap::from([
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
        .into())
    }
}
