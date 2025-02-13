use di::*;
use rustium::{
    authentication::{auth_user::AuthUser, hash_password},
    axum::async_trait,
    datastore::{
        idb::IdThing,
        object::{TakeX, TakeXImpl},
    },
    prelude::*,
    result::RustiumResult,
    service::RustiumService,
    surrealdb::sql::{Number, Object, Value},
};

use crate::users::{
    dtos::storage::{CreateUserDTO, UpdateUserDTO},
    model::{User, UserType},
    repository::IUserRepository,
};

/// conversion impls
impl TryFrom<Wrap<Value>> for UserType {
    type Error = RustiumError;
    fn try_from(val: Wrap<Value>) -> RustiumResult<UserType> {
        match val.0 {
            Value::Number(val) => match val {
                Number::Int(0) => Ok(UserType::Dev),
                Number::Int(1) => Ok(UserType::Admin),
                Number::Int(2) => Ok(UserType::Agent),
                Number::Int(3) => Ok(UserType::Basic),
                _ => Ok(UserType::Basic),
            },
            _ => Err(RustiumError::PropertyNotFound(String::from("user_type"))),
        }
    }
}

impl TakeXImpl<UserType> for Object {
    fn take_x_impl(&mut self, k: &str) -> RustiumResult<Option<UserType>> {
        let v = self.remove(k).map(|v| Wrap(v).try_into());
        match v {
            None => Ok(None),
            Some(Ok(val)) => Ok(Some(val)),
            Some(Err(e)) => Err(e),
        }
    }
}

/// Display
impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "User {}", self.id)?;
        Ok(())
    }
}

/// Auth User implementation
impl AuthUser for User {
    fn get_id(&self) -> String {
        self.id.clone()
    }

    fn get_username(&self) -> String {
        self.email.clone()
    }

    fn is_admin(&self) -> bool {
        self.user_type == UserType::Admin
    }
}

/// from SurrealDB object to User
impl TryFrom<Object> for User {
    type Error = RustiumError;
    fn try_from(mut val: Object) -> RustiumResult<User> {
        let obj = User {
            id: val.take_x_val("id")?,
            name: val.take_x_val("name")?,
            email: val.take_x_val("email")?,
            password: val.take_x_val("password")?,
            user_type: val.take_x_val("user_type")?,
        };

        Ok(obj)
    }
}

#[async_trait]
pub trait IUserService: RustiumService {
    async fn get(&self, user_id: String) -> RustiumResult<User>;
    async fn check_exists(&self, email: String, name: String) -> bool;
    async fn get_by_email(&self, email: String) -> RustiumResult<User>;
    async fn create(&self, data: CreateUserDTO) -> RustiumResult<IdThing>;
    async fn update(&self, id: &str, data: UpdateUserDTO) -> RustiumResult<IdThing>;
    async fn delete(&self, id: &str) -> RustiumResult<bool>;
    async fn is_password_match(&self, user: User, password: &str) -> RustiumResult<bool>;
    fn as_boxed(&mut self) -> RustiumResult<Option<Box<&mut UserService>>>;
}

/// user service interface
#[injectable(IUserService)]
pub struct UserService {
    user_repository: Ref<dyn IUserRepository>,
}

#[async_trait]
impl RustiumService for UserService {
    fn as_rustium(&self) -> RustiumResult<Option<Box<&dyn RustiumService>>> {
        Ok(Some(Box::new(self)))
    }

    async fn init(&mut self) -> RustiumResult<()> {
        Ok(())
    }

    async fn run(&self) -> RustiumResult<()> {
        Ok(())
    }
}

#[async_trait]
impl IUserService for UserService {
    fn as_boxed(&mut self) -> RustiumResult<Option<Box<&mut UserService>>> {
        Ok(Some(Box::new(self)))
    }

    async fn get(&self, user_id: String) -> RustiumResult<User> {
        let user_key = if user_id.starts_with("users:") {
            user_id
        } else {
            format!("users:{}", user_id)
        };

        let user = self.user_repository.get_user(&user_key).await?;

        Ok(user.clone())
    }

    async fn check_exists(&self, email: String, name: String) -> bool {
        println!("check user exists");
        match self.user_repository.get_user_by_email(&email).await {
            Ok(_) => true,
            Err(_) => match self.user_repository.get_user_by_name(&name).await {
                Ok(_) => true,
                Err(_) => false,
            },
        }
    }

    async fn get_by_email(&self, email: String) -> RustiumResult<User> {
        self.user_repository.get_user_by_email(&email).await
    }

    async fn create(&self, data: CreateUserDTO) -> RustiumResult<IdThing> {
        self.user_repository.create_user(data).await
    }

    async fn update(&self, id: &str, data: UpdateUserDTO) -> RustiumResult<IdThing> {
        self.user_repository.update_user(id, data).await
    }

    async fn delete(&self, id: &str) -> RustiumResult<bool> {
        self.user_repository.delete_user(id).await
    }

    async fn is_password_match(&self, user: User, password: &str) -> RustiumResult<bool> {
        Ok(user.password == hash_password(password.to_string())?)
    }
}
