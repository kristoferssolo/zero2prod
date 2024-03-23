use std::net::SocketAddr;

use tokio::net::TcpListener;
use zero2prod::app;

#[tokio::test]
async fn health_check() {
    spawn_app();
    let client = reqwest::Client::new();
    let response = client
        .get("http://localhost:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request.");
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    let _ = tokio::spawn(async move {
        let listener = TcpListener::bind(addr).await.unwrap();
        axum::serve(listener, app()).await.unwrap();
    });
}
