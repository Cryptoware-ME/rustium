use rustium::{prelude::*, RouterMap};

pub fn routes_map() -> RouterMap {
    map! [
        "users" => crate::users::handlers::create_routes()
    ]
}
