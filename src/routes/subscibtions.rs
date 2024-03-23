use axum::{http::StatusCode, response::IntoResponse, Form};
use serde::Deserialize;
#[derive(Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

pub async fn subscribe(Form(form): Form<FormData>) -> impl IntoResponse {
    StatusCode::OK
}
