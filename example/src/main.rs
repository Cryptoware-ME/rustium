pub mod router;
pub mod users;

use rustium::{
    datastore::surreal_dal::SurrealDAL, di::*, settings::RustiumSettings, tokio, RustiumApp,
};

use crate::{router::create_routes, users::service::UserService};

#[tokio::main]
async fn main() {
    let mut provider = ServiceCollection::new();
    provider
        .add(RustiumSettings::singleton())
        .add(SurrealDAL::singleton().depends_on(exactly_one::<RustiumSettings>()))
        .add(UserService::singleton().depends_on(exactly_one::<SurrealDAL>()));

    let app = create_routes();

    RustiumApp::launch(provider, app)
        .await
        .expect("App launch error")
}
