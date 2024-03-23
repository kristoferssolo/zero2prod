use std::net::SocketAddr;

use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use zero2prod::{configuation::get_configuration, routes::route};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuation = get_configuration().expect("Failed to read configuation.");
    let pool = PgPoolOptions::new()
        .connect(&configuation.database.to_string())
        .await
        .expect("Failed to connect to Postgres.");
    let addr = SocketAddr::from(([127, 0, 0, 1], configuation.application_port));
    let listener = TcpListener::bind(addr)
        .await
        .expect("Failed to bind random port");
    axum::serve(listener, route(pool)).await
}
