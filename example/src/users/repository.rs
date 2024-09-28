use rustium::{
    datastore::{
        idb::{IRustiumDb, IdThing},
        object::TakeX,
        repositories::base::{create, delete, get, list, update},
    },
    modql::filter::{ListOptions, OpValString, OpValsInt64, OpValsString},
    prelude::*,
    surrealdb::sql::Object,
};
use std::sync::Arc;

use crate::users::{dtos::*, model::User};

// region: Public Functions

pub async fn get_user(dal: &Arc<dyn IRustiumDb>, id: &str) -> RustiumResult<User> {
    get(dal, id).await
}

pub async fn get_user_by_name(ctx: &Arc<Database>, name: &str) -> RustiumResult<User> {
    match list(
        ctx,
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
        Err(_) => Err(Error::not_found()),
    }
}

pub async fn get_user_by_email(ctx: &Arc<Database>, email: &str) -> RustiumResult<User> {
    match list(
        ctx,
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
        Err(_) => Err(Error::not_found()),
    }
}

pub async fn get_user_by_google_id(
    ctx: &Arc<Database>,
    id: String,
    token: String,
) -> RustiumResult<User> {
    match list(
        ctx,
        "users",
        Some(
            UserFilter {
                email: None::<OpValsString>,
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
        Err(_) => Err(Error::not_found()),
    }
}

pub async fn create_user(ctx: &Arc<Database>, data: CreateUserDTO) -> RustiumResult<IdThing> {
    create(ctx, "users".into(), data).await
}

pub async fn update_user(
    ctx: &Arc<Database>,
    id: &str,
    data: UpdateUserDTO,
) -> RustiumResult<IdThing> {
    update(ctx, id, data).await?
}

pub async fn delete_user(ctx: &Arc<Database>, id: &str) -> RustiumResult<String> {
    delete(ctx, id).await
}
// endregion: Public Functions
