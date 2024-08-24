use crate::helpers::spawn_app;
use reqwest::Client;
use sqlx::{Connection, PgConnection};
use zero2prod::config::get_config;

#[tokio::test]
async fn subscribe_returns_200_for_valid_form_data() {
    let app = spawn_app().await;
    let body = "name=Kristofers%20Solo&email=dev%40kristofers.solo";

    let config = get_config().expect("Failed to read configuration.");
    let mut connection = PgConnection::connect_with(&config.database.with_db())
        .await
        .expect("Failed to connect to Postgres.");

    let response = app.post_subscription(body.into()).await;

    assert_eq!(200, response.status().as_u16());
    let saved = sqlx::query!(
        r#"
        SELECT email, name
          FROM subscriptions
        "#
    )
    .fetch_one(&mut connection)
    .await
    .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.name, "Kristofers Solo");
    assert_eq!(saved.email, "dev@kristofers.solo");
}

#[tokio::test]
async fn subscribe_returns_400_when_data_is_missing() {
    let app = spawn_app().await;

    let test_cases = vec![
        ("name=krisotfers%20solo", "missing the email"),
        ("email=dev%40kristofers.solo", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = app.post_subscription(invalid_body.into()).await;

        assert_eq!(
            422,
            response.status().as_u16(),
            "The API did not call with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}

#[tokio::test]
async fn subscribe_returns_400_when_fields_are_present_but_invalid() {
    let app = spawn_app().await;
    let test_cases = vec![
        ("name=&email=dev%40kristofers.solo", "empty name"),
        ("name=kristofers%20solo&email=", "empty email"),
        ("name=solo&email=definetely-not-an-email", "invalid email"),
    ];

    for (body, description) in test_cases {
        let response = app.post_subscription(body.into()).await;
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not return 400 Bad Request when the payload was {}.",
            description
        );
    }
}
