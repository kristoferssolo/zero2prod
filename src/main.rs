use std::net::SocketAddr;

use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use zero2prod::{
    configuation::get_configuration,
    routes::route,
    telemetry::{get_subscriber, init_subscriber},
};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("zero2prod", "info");
    init_subscriber(subscriber);
    let configuation = get_configuration().expect("Failed to read configuation.");
    let pool = PgPoolOptions::new()
        .connect(&configuation.database.to_string())
        .await
        .expect("Failed to connect to Postgres.");
    let addr = SocketAddr::from(([127, 0, 0, 1], configuation.application_port));
    let listener = TcpListener::bind(addr)
        .await
        .expect("Failed to bind port 8000.");

    axum::serve(listener, route(pool)).await
}
