use rustium::{
    axum::Router, datastore::surreal_dal::SurrealDAL, di::*, settings::RustiumSettings, tokio,
    RustiumApp,
};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let provider = ServiceCollection::new()
        .add(RustiumSettings::singleton())
        .add(SurrealDAL::singleton().depends_on(RustiumSettings));

    let app: Router<()>;

    RustiumApp::launch(provider, app)
        .await
        .expect("App launch error")
}
