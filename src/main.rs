use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use zero2prod::{
    config::get_config,
    routes::route,
    telemetry::{get_subscriber, init_subscriber},
};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("zero2prod", "info", std::io::stdout);
    init_subscriber(subscriber);
    let config = get_config().expect("Failed to read configuation.");
    let pool = PgPoolOptions::new().connect_lazy_with(config.database.with_db());
    let addr = format!("{}:{}", config.application.host, config.application.port);
    let listener = TcpListener::bind(addr)
        .await
        .expect("Failed to bind port 8000.");

    axum::serve(listener, route(pool)).await
}
