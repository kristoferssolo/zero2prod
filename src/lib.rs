use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Form, Router,
};
use serde::Deserialize;

pub fn app() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/:name", get(greet))
        .route("/health_check", get(health_check))
        .route("/subscribtions", post(subscribe))
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

#[derive(Deserialize)]
struct FormData {
    name: String,
    email: String,
}

async fn subscribe(Form(form): Form<FormData>) -> impl IntoResponse {
    StatusCode::OK
}
