mod health_check;
mod subscibtions;
use axum::{
    routing::{get, post},
    Router,
};

pub use health_check::*;
pub use subscibtions::*;

pub fn route() -> Router {
    Router::new()
        .route("/health_check", get(health_check))
        .route("/subscribtions", post(subscribe))
}
