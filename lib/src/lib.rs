pub mod authentication;
pub mod datastore;
pub mod error;
pub mod logging;
pub mod prelude;
pub mod response;
pub mod result;
pub mod service;
pub mod settings;

/// re-exporting packages
pub use argon2;
pub use axum;
pub use di;
pub use di_axum;
pub use http;
pub use modql;
pub use serde;
pub use serde_derive;
pub use serde_json;
pub use surrealdb;
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

use crate::{datastore::idb::IRustiumDb, prelude::*, settings::IRustiumSettings};

pub struct RustiumApp {}

impl RustiumApp {
    pub async fn launch(provider: ServiceCollection, app: Router<()>) -> RustiumResult<()> {
        // grab listener and define socket
        let address = SocketAddr::from(([0, 0, 0, 0], 8080));
        let listener = TcpListener::bind(address)
            .await
            .expect("Failed to attach to port");

        println!("Building Provider");

        // initialize services
        let built_provider = provider.build_provider()?;
        // init settings service
        let settings = match built_provider.get_mut::<dyn IRustiumSettings>() {
            Some(db) => db,
            None => {
                return Err(RustiumError::ServiceNotFound(
                    "The required database service is missing".into(),
                ))
            }
        };

        let mut settings = match settings.write() {
            Ok(setting) => setting,
            Err(_) => {
                return Err(RustiumError::PoisonedRef(
                    "Settings Service is poisoned".into(),
                ))
            }
        };

        settings
            .as_rustium()
            .expect("Settings Service is poisoned")
            .expect("Settings Service is poisoned")
            .init()
            .await?;

        // init db service
        let db = match built_provider.get_mut::<dyn IRustiumDb>() {
            Some(db) => db,
            None => {
                return Err(RustiumError::ServiceNotFound(
                    "The required database service is missing".into(),
                ))
            }
        };

        let mut dbi = match db.write() {
            Ok(db) => db,
            Err(_) => return Err(RustiumError::PoisonedRef("DB Service is poisoned".into())),
        };

        dbi.as_rustium()
            .expect("DB Service is poisoned")
            .expect("DB Service is poisoned")
            .init()
            .await?;

        let trace_level = match settings.get_logger()?.level.as_str() {
            "debug" => tracing::Level::DEBUG,
            "info" => tracing::Level::INFO,
            "error" => tracing::Level::ERROR,
            "trace" => tracing::Level::TRACE,
            "warn" => tracing::Level::WARN,
            _ => tracing::Level::DEBUG,
        };

        // web app launch
        axum::serve(
            listener,
            app.layer(
                TraceLayer::new_for_http()
                    .make_span_with(DefaultMakeSpan::new().include_headers(true))
                    .on_request(DefaultOnRequest::new().level(trace_level))
                    .on_response(DefaultOnResponse::new().level(trace_level)),
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
