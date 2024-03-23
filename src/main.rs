use std::net::SocketAddr;

use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    let listener = TcpListener::bind(addr)
        .await
        .expect("Failed to bind random port");
    axum::serve(listener, zero2prod::app()).await.unwrap();
}
