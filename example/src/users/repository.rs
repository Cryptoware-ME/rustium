use rustium::{
    datastore::{
        idb::IdThing,
        repositories::base::{create, delete, get, list, update},
        surreal_dal::SurrealDAL,
    },
    modql::filter::{ListOptions, OpValString, OpValsInt64, OpValsString},
    prelude::*,
};
use std::sync::Arc;

use crate::users::{dtos::storage::*, model::User};

// region: Public Functions

pub async fn get_user(dal: Arc<SurrealDAL>, id: &str) -> RustiumResult<User> {
    get(dal, id).await
}

pub async fn get_user_by_name(dal: Arc<SurrealDAL>, name: &str) -> RustiumResult<User> {
    match list::<User, SurrealDAL>(
        dal,
        "users",
        Some(
            UserFilter {
                email: None::<OpValsString>,
                id: None::<OpValsString>,
                name: Some(OpValsString(vec![OpValString::Eq(name.into())])),
                user_type: None::<OpValsInt64>,
            }
            .into(),
        ),
        ListOptions::default(),
    )
    .await
    {
        Ok(res) => Ok(res[0].clone()),
        Err(_) => Err(RustiumError::not_found("users")),
    }
}

pub async fn get_user_by_email(dal: Arc<SurrealDAL>, email: &str) -> RustiumResult<User> {
    match list::<User, SurrealDAL>(
        dal,
        "users",
        Some(
            UserFilter {
                email: Some(OpValsString(vec![OpValString::Eq(email.into())])),
                id: None::<OpValsString>,
                name: None::<OpValsString>,
                user_type: None::<OpValsInt64>,
            }
            .into(),
        ),
        ListOptions::default(),
    )
    .await
    {
        Ok(res) => Ok(res[0].clone()),
        Err(_) => Err(RustiumError::not_found("users")),
    }
}

pub async fn create_user(dal: Arc<SurrealDAL>, data: CreateUserDTO) -> RustiumResult<IdThing> {
    create(dal, "users".into(), data).await
}

pub async fn update_user(
    dal: Arc<SurrealDAL>,
    id: &str,
    data: UpdateUserDTO,
) -> RustiumResult<IdThing> {
    update(dal, id, data).await
}

pub async fn delete_user(dal: Arc<SurrealDAL>, id: &str) -> RustiumResult<bool> {
    delete(dal, id).await
}
// endregion: Public Functions
