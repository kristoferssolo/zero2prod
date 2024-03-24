mod health_check;
mod subscibtions;

use axum::{
    routing::{get, post},
    Router,
};

pub use health_check::*;
use sqlx::PgPool;
pub use subscibtions::*;

pub fn route(state: PgPool) -> Router {
    Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .with_state(state)
}
