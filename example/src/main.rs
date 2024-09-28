pub mod notifications;
pub mod router;
pub mod users;

use rustium::{
    axum::Router, datastore::surreal_dal::SurrealDAL, di::*, settings::RustiumSettings, tokio,
    RustiumApp,
};

#[tokio::main]
async fn main() {
    let mut provider = ServiceCollection::new();
    provider
        .add(RustiumSettings::singleton())
        .add(SurrealDAL::singleton());

    let app = Router::default();

    RustiumApp::launch(provider, app)
        .await
        .expect("App launch error")
}
