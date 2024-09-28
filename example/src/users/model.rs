use rustium::serde::{Deserialize, Serialize};

// use crate::{};

// region: Structs/Enums
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub user_type: UserType,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum UserType {
    Dev,
    Admin,
    Agent,
    Basic,
}
// endregion: Structs/Enums
