/// exporting modules
pub mod authentication;
pub mod datastore;
pub mod error;
pub mod logging;
pub mod prelude;
pub mod response;
pub mod result;
pub mod service;
pub mod settings;

/// exporting packages
pub use argon2;
pub use axum;
pub use di;
pub use di_axum;
pub use http;
pub use modql;
pub use serde;
pub use serde_derive;
pub use serde_json;
use settings::interface::IRustiumSettings;
pub use surrealdb;
pub use tokio;
pub use tower_http;

use axum::Router;
use di::ServiceProvider;
use di_axum::RouterServiceProviderExtensions;
use http::header::{HeaderName, AUTHORIZATION};
use std::{collections::BTreeMap, net::SocketAddr};
use tokio::{net::TcpListener, sync::Notify};
use tower_http::{
    compression::CompressionLayer,
    cors::CorsLayer,
    propagate_header::PropagateHeaderLayer,
    sensitive_headers::SetSensitiveHeadersLayer,
    trace::{DefaultMakeSpan, DefaultOnFailure, DefaultOnRequest, DefaultOnResponse, TraceLayer},
};

use crate::prelude::*;

pub type RouterMap = BTreeMap<&'static str, Router<()>>;

pub struct RustiumApp {}

impl RustiumApp {
    pub async fn launch(provider: ServiceProvider, routes: RouterMap) -> RustiumResult<()> {
        // grab listener and define socket
        let address = SocketAddr::from(([0, 0, 0, 0], 8080));

        println!("Server listening on {}", &address);

        let listener = TcpListener::bind(address)
            .await
            .expect("Failed to attach to port");

        // get settings
        let settings = match provider.get::<dyn IRustiumSettings>() {
            Some(set) => set,
            None => {
                return Err(RustiumError::ServiceNotFound(
                    "The required settings service is missing".into(),
                ))
            }
        };

        let trace_level = match settings.get_logger()?.level.as_str() {
            "debug" => tracing::Level::DEBUG,
            "info" => tracing::Level::INFO,
            "error" => tracing::Level::ERROR,
            "trace" => tracing::Level::TRACE,
            "warn" => tracing::Level::WARN,
            _ => tracing::Level::DEBUG,
        };

        // merge routes
        let mut app: Router<()> = Router::new();

        for (k, v) in routes {
            app = app.nest(&f!("/{}", k), v);
        }

        app = Router::new().nest(&f!("/v{}", settings.get_api()?.version), app);

        // web app launch
        axum::serve(
            listener,
            app.layer(
                TraceLayer::new_for_http()
                    .make_span_with(DefaultMakeSpan::new().include_headers(true))
                    .on_request(DefaultOnRequest::new().level(trace_level))
                    .on_response(DefaultOnResponse::new().level(trace_level))
                    .on_failure(DefaultOnFailure::new().level(trace_level)),
            )
            .layer(SetSensitiveHeadersLayer::new(std::iter::once(
                AUTHORIZATION,
            )))
            .layer(CompressionLayer::new())
            .layer(PropagateHeaderLayer::new(HeaderName::from_static(
                "x-request-id",
            )))
            .layer(CorsLayer::permissive())
            .with_provider(provider)
            .into_make_service(),
        )
        .await
        .expect("Server should launch");

        Notify::new().notified().await;

        Ok(())
    }
}
