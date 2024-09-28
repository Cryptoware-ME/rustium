pub mod authentication;
pub mod datastore;
pub mod error;
pub mod logging;
pub mod prelude;
pub mod response;
pub mod result;
pub mod service;
pub mod settings;

/// rexporting packages
pub use axum;
use datastore::idb_dal::IDbDal;
pub use di;
pub use di_axum;
pub use http;
pub use tokio;
pub use tower_http;

use axum::Router;
use di::ServiceCollection;
use di_axum::RouterServiceProviderExtensions;
use http::header::{HeaderName, AUTHORIZATION};
use std::net::SocketAddr;
use tokio::{net::TcpListener, sync::Notify};
use tower_http::{
    compression::CompressionLayer,
    cors::CorsLayer,
    propagate_header::PropagateHeaderLayer,
    sensitive_headers::SetSensitiveHeadersLayer,
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
};

use crate::prelude::*;

pub struct RustiumApp {}

impl RustiumApp {
    pub async fn launch(provider: &mut ServiceCollection, app: Router<()>) -> RustiumResult<()> {
        // grab listener and define socket
        let address = SocketAddr::from(([0, 0, 0, 0], 8080));
        let listener = TcpListener::bind(address)
            .await
            .expect("Failed to attach to port");

        // initialize services
        let built_provider = provider.build_provider()?;

        // init db service
        let db = match built_provider.get_mut::<dyn IDbDal>() {
            Some(db) => db,
            None => {
                return Err(RustiumError::ServiceNotFound(
                    "The required databse service is missing".into(),
                ))
            }
        };

        let mut dbi = match db.write() {
            Ok(db) => db,
            Err(_) => return Err(RustiumError::PoisonedRef("DB Service is poisoned".into())),
        };

        dbi.as_rustium().expect("").expect("").init().await?;

        // // init settings service
        // let db = match built_provider.get_mut::<dyn IDbDal>() {
        //     Some(db) => db,
        //     None => {
        //         return Err(RustiumError::ServiceNotFound(
        //             "The required databse service is missing".into(),
        //         ))
        //     }
        // };

        // web app launch
        axum::serve(
            listener,
            app.layer(
                TraceLayer::new_for_http()
                    .make_span_with(DefaultMakeSpan::new().include_headers(true))
                    .on_request(DefaultOnRequest::new().level(tracing::Level::DEBUG))
                    .on_response(DefaultOnResponse::new().level(tracing::Level::DEBUG)),
            )
            .layer(SetSensitiveHeadersLayer::new(std::iter::once(
                AUTHORIZATION,
            )))
            .layer(CompressionLayer::new())
            .layer(PropagateHeaderLayer::new(HeaderName::from_static(
                "x-request-id",
            )))
            .layer(CorsLayer::permissive())
            .with_provider(built_provider)
            .into_make_service(),
        )
        .await
        .expect("Failed to start server");

        Notify::new().notified().await;

        Ok(())
    }
}
