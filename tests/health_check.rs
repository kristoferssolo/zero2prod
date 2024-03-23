use reqwest::Client;
use sqlx::{postgres::PgPoolOptions, Connection, Executor, PgConnection, PgPool};
use tokio::net::TcpListener;
use uuid::Uuid;
use zero2prod::{
    configuation::{get_configuration, DatabaseSettings},
    routes::route,
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
    let configuration = get_configuration().expect("Failed to read configuration.");
    let db_url = configuration.database.to_string();
    let mut connection = PgConnection::connect(&db_url)
        .await
        .expect("Failed to connect to Postgres.");
    let client = Client::new();

    let body = "name=Kristofers%20Solo&email=dev%40kristofers.solo";
    let response = client
        .post(&format!("{}/subscribtions", &app.address))
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

    assert_eq!(saved.email, "dev@kristofers.solo");
    assert_eq!(saved.name, "Kristofers Solo");
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
            .post(&format!("{}/subscribtions", &app.address))
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

async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);
    let mut config = get_configuration().expect("Failed to read configuration.");

    config.database.database_name = Uuid::new_v4().to_string();
    let pool = configure_database(&config.database).await;
    let pool_clone = pool.clone();
    let _ = tokio::spawn(async move {
        axum::serve(listener, route(pool_clone))
            .await
            .expect("Failed to bind address.")
    });
    TestApp { address, pool }
}

async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect(&config.to_string_no_db())
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

    let pool = PgPoolOptions::new()
        .connect(&config.to_string())
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
