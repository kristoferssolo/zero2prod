use std::{net::SocketAddr, time::Duration};

use axum::{
    body::Bytes,
    extract::MatchedPath,
    http::{HeaderMap, Request},
    response::Response,
};
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};
use tracing::{info_span, subscriber::set_global_default, Span};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};
use zero2prod::{configuation::get_configuration, routes::route};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    LogTracer::init().expect("Failed to set logger");
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into());
    let formatting_layer = BunyanFormattingLayer::new("zero2prod".into(), std::io::stdout);
    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);

    set_global_default(subscriber).expect("Failed to set subscriber.");

    let configuation = get_configuration().expect("Failed to read configuation.");
    let pool = PgPoolOptions::new()
        .connect(&configuation.database.to_string())
        .await
        .expect("Failed to connect to Postgres.");
    let addr = SocketAddr::from(([127, 0, 0, 1], configuation.application_port));
    let listener = TcpListener::bind(addr)
        .await
        .expect("Failed to bind port 8000.");

    let route = route(pool)
        .layer(TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
            let matched_path = request.extensions().get::<MatchedPath>().map(MatchedPath::as_str);
            info_span!("http-request", method = ?request.method(), matched_path, some_other_field= tracing::field::Empty,)
        })
        .on_request(|_request: &Request<_>, _span: &Span | {})
        .on_response(|_response: &Response<_>, _latency: Duration, _span:&Span|{})
        .on_body_chunk(|_chunk: &Bytes, _latency: Duration, _span:&Span|{})
        .on_eos(|_trailers: Option<&HeaderMap>, _stream_duration: Duration, _span:&Span| {})
        .on_failure(|_error: ServerErrorsFailureClass, _latency: Duration, _span:&Span| {})
        );
    axum::serve(listener, route).await
}
