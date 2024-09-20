use tower_http::{
    classify::{ServerErrorsAsFailures, SharedClassifier},
    trace::{DefaultMakeSpan, DefaultOnFailure, DefaultOnRequest, DefaultOnResponse, TraceLayer},
};
use tracing::Level;

pub fn setup_tracing(
    on_request_level: Level,
    on_response_level: Level,
    on_failure_level: Level,
) -> TraceLayer<SharedClassifier<ServerErrorsAsFailures>> {
    TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::new().include_headers(true))
        .on_request(DefaultOnRequest::new().level(on_request_level))
        .on_response(DefaultOnResponse::new().level(on_response_level))
        .on_failure(DefaultOnFailure::new().level(on_failure_level))
}
