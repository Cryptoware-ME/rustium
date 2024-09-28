use rustium::axum::Router;

use crate::users::handlers::create_routes as create_user_routes;

pub fn create_routes() -> Router<()> {
    Router::new().nest("/users", create_user_routes())
}
