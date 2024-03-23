use reqwest::Client;
use sqlx::{Connection, PgConnection};
use tokio::net::TcpListener;
use zero2prod::{configuation::get_configuration, routes::route};

#[tokio::test]
async fn health_check() {
    let address = spawn_app().await;
    let url = format!("{}/health_check", &address);
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
    let address = spawn_app().await;
    let configuration = get_configuration().expect("Failed to read configuration.");
    let db_url = configuration.database.to_string();
    let mut connection = PgConnection::connect(&db_url)
        .await
        .expect("Failed to connect to Postgres.");
    let client = Client::new();

    let body = "name=Kristofers%20Solo&email=dev%40kristofers.solo";
    let response = client
        .post(&format!("{}/subscribtions", &address))
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
    let address = spawn_app().await;
    let client = Client::new();

    let test_cases = vec![
        ("name=krisotfers%20solo", "missing the email"),
        ("email=dev%40kristofers.solo", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscribtions", &address))
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

async fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let _ = tokio::spawn(async move { axum::serve(listener, route()).await.unwrap() });
    format!("http://127.0.0.1:{}", port)
}
