pub mod create;
pub mod get;
pub mod login;

use rustium::axum::{
    routing::{get, post},
    Router,
};

use crate::users::handlers::{create::create_user, get::get_user, login::authenticate_user};

pub fn create_routes() -> Router<()> {
    Router::new()
        .route("/", post(create_user))
        .route("/login", post(authenticate_user))
        .route("/:id", get(get_user))
}
