use axum::{extract::Path, http::StatusCode, response::IntoResponse, routing::get, Router};
use tower_http::trace::TraceLayer;

pub fn app() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/:name", get(greet))
        .route("/health_check", get(health_check))
        .layer(TraceLayer::new_for_http())
}

async fn root() -> impl IntoResponse {
    "Hello, world!"
}

async fn greet(Path(name): Path<String>) -> impl IntoResponse {
    format!("Hello, {name}!")
}
async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}
