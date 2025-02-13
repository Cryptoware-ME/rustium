use di::*;
use rustium::{
    axum::async_trait,
    datastore::{
        idb::{IRustiumDb, IdThing},
        repositories::base::{create, delete, get, list, update},
    },
    modql::filter::{ListOptions, OpValString, OpValsInt64, OpValsString},
    prelude::*,
    service::RustiumService,
};

use crate::users::{dtos::storage::*, model::User};

#[async_trait]
pub trait IUserRepository: RustiumService {
    async fn get_user(&self, id: &str) -> RustiumResult<User>;
    async fn get_user_by_name(&self, name: &str) -> RustiumResult<User>;
    async fn get_user_by_email(&self, email: &str) -> RustiumResult<User>;
    async fn create_user(&self, data: CreateUserDTO) -> RustiumResult<IdThing>;
    async fn update_user(&self, id: &str, data: UpdateUserDTO) -> RustiumResult<IdThing>;
    async fn delete_user(&self, id: &str) -> RustiumResult<bool>;
}

#[injectable(IUserRepository)]
pub struct UserRepository {
    db: Ref<dyn IRustiumDb>,
}

#[async_trait]
impl IUserRepository for UserRepository {
    async fn get_user(&self, id: &str) -> RustiumResult<User> {
        get(self.db.clone(), id).await
    }

    async fn get_user_by_name(&self, name: &str) -> RustiumResult<User> {
        match list::<User, _>(
            self.db.clone(),
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

    async fn get_user_by_email(&self, email: &str) -> RustiumResult<User> {
        match list::<User, _>(
            self.db.clone(),
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

    async fn create_user(&self, data: CreateUserDTO) -> RustiumResult<IdThing> {
        create(self.db.clone(), "users".into(), data).await
    }

    async fn update_user(&self, id: &str, data: UpdateUserDTO) -> RustiumResult<IdThing> {
        update(self.db.clone(), id, data).await
    }

    async fn delete_user(&self, id: &str) -> RustiumResult<bool> {
        delete(self.db.clone(), id).await
    }
}

#[async_trait]
impl RustiumService for UserRepository {
    async fn init(&mut self) -> RustiumResult<()> {
        Ok(())
    }

    async fn run(&self) -> RustiumResult<()> {
        Ok(())
    }

    fn as_rustium(&self) -> RustiumResult<Option<Box<&dyn RustiumService>>> {
        Ok(Some(Box::new(self)))
    }
}
