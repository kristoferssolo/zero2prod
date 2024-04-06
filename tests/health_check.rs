use once_cell::sync::Lazy;
use reqwest::Client;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use tokio::net::TcpListener;
use uuid::Uuid;
use zero2prod::{
    config::{get_config, DatabaseSettings},
    email_client::EmailClient,
    routes::route,
    telemetry::{get_subscriber, init_subscriber},
};

#[tokio::test]
async fn health_check() {
    let app = spawn_app().await;
    let url = format!("{}/health_check", &app.address);
    let client = Client::new();
    let response = client
        .get(&url)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_200_for_valid_form_data() {
    let app = spawn_app().await;
    let body = "name=Kristofers%20Solo&email=dev%40kristofers.solo";

    let config = get_config().expect("Failed to read configuration.");
    let mut connection = PgConnection::connect_with(&config.database.with_db())
        .await
        .expect("Failed to connect to Postgres.");
    let client = Client::new();

    let response = client
        .post(&format!("{}/subscriptions", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

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
    let client = Client::new();

    let test_cases = vec![
        ("name=krisotfers%20solo", "missing the email"),
        ("email=dev%40kristofers.solo", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

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
    let client = Client::new();
    let test_cases = vec![
        ("name=&email=dev%40kristofers.solo", "empty name"),
        ("name=kristofers%20solo&email=", "empty email"),
        ("name=solo&email=definetely-not-an-email", "invalid email"),
    ];

    for (body, description) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request.");
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not return 400 Bad Request when the payload was {}.",
            description
        );
    }
}

static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "trace";
    let subscriber_name = "test";
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(default_filter_level, subscriber_name, std::io::sink);
        init_subscriber(subscriber);
    }
});

async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let mut config = get_config().expect("Failed to read configuration.");

    config.database.database_name = Uuid::new_v4().to_string();

    let pool = configure_database(&config.database).await;
    let pool_clone = pool.clone();

    let sender_email = config
        .email_client
        .sender()
        .expect("Invalid sender email adress");
    let email_client = EmailClient::new(
        config.email_client.base_url,
        sender_email,
        config.email_client.auth_token,
    );

    let _ = tokio::spawn(async move {
        axum::serve(listener, route(pool_clone, email_client))
            .await
            .expect("Failed to bind address.")
    });
    TestApp { address, pool }
}

async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connect to Postgres.");

    connection
        .execute(
            format!(
                r#"
                  CREATE DATABASE "{}"
                "#,
                config.database_name
            )
            .as_str(),
        )
        .await
        .expect("Failed to create database.");

    let pool = PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect to Postgres.");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to migrate database");

    pool
}

pub struct TestApp {
    pub address: String,
    pub pool: PgPool,
}
