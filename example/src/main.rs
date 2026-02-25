pub mod auth;
pub mod router;
pub mod users;

use rustium::{
    datastore::{idb::IRustiumDb, surreal_dal::SurrealDAL},
    di::*,
    settings::{interface::IRustiumSettings, RustiumSettings},
    tokio, RustiumApp,
};
// use users::dtos::{requests::CreateUserRequestDTO, storage::CreateUserDTO};

use crate::{
    auth::service::AuthService,
    router::routes_map,
    users::{
        repository::{IUserRepository, UserRepository},
        service::{IUserService, UserService},
    },
};

#[tokio::main]
async fn main() {
    let provider = ServiceCollection::new()
        // base dependencies (singletons: shared across all requests)
        .add(RustiumSettings::singleton())
        .add(SurrealDAL::singleton())
        // User repository and service
        .add(UserRepository::scoped())
        .add(UserService::scoped())
        // essential auth service implementation
        .add(AuthService::scoped())
        // building provider
        .build_provider()
        .expect("Service provider unintialized");

    // let db = match provider.get::<dyn IRustiumDb>() {
    //     Some(d) => d,
    //     None => panic!("at the disco"),
    // };

    // let cu: CreateUserDTO = (CreateUserRequestDTO {
    //     name: "Jad".to_string(),
    //     email: "jad.jabbour@cryptoware.me".to_string(),
    //     password: "112233".to_string(),
    // })
    // .into();

    // let _ = db
    //     .exec_create("users", cu.try_into().expect("..."))
    //     .await
    //     .expect("...");

    // println!("done");

    RustiumApp::launch(provider, routes_map())
        .await
        .expect("App launch error")
}
