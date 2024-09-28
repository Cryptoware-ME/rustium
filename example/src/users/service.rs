use di::*;
use rustium::{
    authentication::auth_user::AuthUser,
    axum::async_trait,
    datastore::{
        idb::{IRustiumDb, IdThing},
        object::{TakeX, TakeXImpl},
        surreal_dal::SurrealDAL,
    },
    prelude::*,
    result::RustiumResult,
    surrealdb::sql::{Number, Object, Value},
};
use std::rc::Rc;

use crate::users::{
    dtos::storage::CreateUserDTO,
    model::{User, UserType},
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
pub trait IUserService {
    async fn get(&self, user_id: String) -> RustiumResult<User>;
    async fn check_exists(&self, email: String, name: String) -> bool;
    async fn get_by_email(&self, email: String) -> RustiumResult<User>;
    async fn create(&self, data: CreateUserDTO) -> RustiumResult<IdThing>;
    async fn update(id: &str, data: UpdateUserDTO) -> RustiumResult<IdThing>;
}

/// user service interface
#[injectable(IUserService)]
pub struct UserService {
    db: Ref<SurrealDAL>,
}

#[async_trait]
impl IUserService for UserService {
    async fn get(&self, user_id: String) -> RustiumResult<User> {}
    async fn check_exists(&self, email: String, name: String) -> bool {}
    async fn get_by_email(&self, email: String) -> RustiumResult<User> {}
    async fn create(&self, data: CreateUserDTO) -> RustiumResult<IdThing> {}
    async fn update(id: &str, data: UpdateUserDTO) -> RustiumResult<IdThing> {}
}

// main service implementation

// impl User {
//     pub async fn get(user_id: String, db: &Arc<Database>) -> RustiumResult<User> {
//         let user_key = if user_id.starts_with("users:") {
//             user_id.clone()
//         } else {
//             format!("users:{}", user_id)
//         };

//         get_user(ctx, &user_key).await
//     }

//     pub async fn check_exists(email: String, name: String, ctx: &Arc<Database>) -> bool {
//         match get_user_by_email(ctx, &email).await {
//             Ok(_) => true,
//             Err(_) => match get_user_by_name(ctx, &name).await {
//                 Ok(_) => true,
//                 Err(_) => false,
//             },
//         }
//     }

//     pub async fn get_by_email(email: String, ctx: &Arc<Database>) -> RustiumResult<User> {
//         get_user_by_email(ctx, &email).await
//     }

//     pub async fn create(data: CreateUserDTO, ctx: &Arc<Database>) -> RustiumResult<IdThing> {
//         create_user(ctx, data).await
//     }

//     pub async fn update(id: &str,data: UpdateUserDTO,ctx: &Arc<Database>,) -> RustiumResult<IdThing> {
//         update_user(ctx, id, data).await
//     }

//     pub async fn delete(id: &str, ctx: &Arc<Database>) -> RustiumResult<String> {
//         delete_user(ctx, id).await
//     }

//     pub fn is_password_match(&self, password: &str) -> RustiumResult<bool> {
//         Ok(self.password == User::hash_password(password.to_string())?)
//     }

//     pub fn hash_password(password: String) -> RustiumResult<String> {
//         let mut output_key_material = [0u8; 32];
//         let v8 = match Argon2::default().hash_password_into(
//             password.into_bytes().as_slice(),
//             b"SuperSecret",
//             &mut output_key_material,
//         ) {
//             Ok(()) => output_key_material.to_vec(),
//             Err(e) => {
//                 println!("Error: {}", e);
//                 [0u8; 16].to_vec()
//             }
//         };

//         Ok(hex::encode(v8))
//     }
// }
