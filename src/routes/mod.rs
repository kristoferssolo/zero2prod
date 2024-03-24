mod health_check;
mod subscibtions;

use std::time::Duration;

use axum::{
    body::Bytes,
    extract::MatchedPath,
    http::{HeaderMap, Request},
    response::Response,
    routing::{get, post},
    Router,
};

pub use health_check::*;
use sqlx::PgPool;
pub use subscibtions::*;
use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};
use tracing::{info_span, Span};

pub fn route(state: PgPool) -> Router {
    Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .with_state(state)
        .layer(TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
            let matched_path = request.extensions().get::<MatchedPath>().map(MatchedPath::as_str);
            info_span!("http-request", method = ?request.method(), matched_path, some_other_field= tracing::field::Empty,)
        })
        .on_request(|_request: &Request<_>, _span: &Span | {})
        .on_response(|_response: &Response<_>, _latency: Duration, _span:&Span|{})
        .on_body_chunk(|_chunk: &Bytes, _latency: Duration, _span:&Span|{})
        .on_eos(|_trailers: Option<&HeaderMap>, _stream_duration: Duration, _span:&Span| {})
        .on_failure(|_error: ServerErrorsFailureClass, _latency: Duration, _span:&Span| {})
        )
}
