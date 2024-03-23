use tokio::net::TcpListener;

#[tokio::test]
async fn health_check() {
    let address = spawn_app().await;
    let url = format!("{}/health_check", &address);
    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

async fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let _ = tokio::spawn(async move { axum::serve(listener, zero2prod::app()).await.unwrap() });
    format!("http://127.0.0.1:{}", port)
}
