pub mod authentication;
pub mod context;
pub mod datastore;
pub mod error;
pub mod logging;
pub mod prelude;
pub mod response;
pub mod result;
pub mod service;
pub mod settings;

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

use crate::{context::AppContext, prelude::*};

pub struct RustiumApp {}

impl RustiumApp {
    pub async fn launch(
        context: AppContext,
        provider: ServiceCollection,
        app: Router<AppContext>,
    ) -> RustiumResult<()> {
        let address = SocketAddr::from(([0, 0, 0, 0], 8080));

        let listener = TcpListener::bind(address)
            .await
            .expect("Failed to attach to port");

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
            .with_state(context)
            .with_provider(provider.build_provider()?)
            .into_make_service(),
        )
        .await
        .expect("Failed to start server");

        Notify::new().notified().await;

        Ok(())
    }
}
