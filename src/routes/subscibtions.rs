use axum::{extract::State, http::StatusCode, response::IntoResponse, Form};
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;
use tracing::{error, info_span, Instrument};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

pub async fn subscribe(
    State(conn): State<PgPool>,
    Form(form): Form<FormData>,
) -> impl IntoResponse {
    let request_id = Uuid::new_v4();
    let request_span = info_span!("Adding a new subscriber", %request_id, subscriber_email = %form.email, subscriber_name = %form.name);
    let _request_span_guard = request_span.enter();
    let query_span = info_span!("Saving new subscriber details in the database");
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions(id, email, name, subscribed_at)
          VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(&conn)
    .instrument(query_span)
    .await
    {
        Ok(_) => StatusCode::OK,
        Err(e) => {
            error!("Failed to execute query: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
