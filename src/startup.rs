use sqlx::{postgres::PgPoolOptions, PgPool};
use tokio::{net::TcpListener, task::JoinHandle};

use crate::{
    config::{DatabaseSettings, Settings},
    email_client::EmailClient,
    routes::route,
};

pub struct Application {
    port: u16,
    server: JoinHandle<Result<(), std::io::Error>>,
}

impl Application {
    pub async fn build(config: Settings) -> Result<Self, std::io::Error> {
        let pool = get_connection_pool(&config.database);

        let sender_email = config
            .email_client
            .sender()
            .expect("Invalid sender email adress");
        let timeout = config.email_client.timeout();
        let email_client = EmailClient::new(
            config.email_client.base_url,
            sender_email,
            config.email_client.auth_token,
            timeout,
        );

        let addr = format!("{}:{}", config.application.host, config.application.port);
        let listener = TcpListener::bind(addr).await?;
        let port = listener.local_addr().unwrap().port();
        let server =
            tokio::spawn(async move { axum::serve(listener, route(pool, email_client)).await });

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await?
    }
}

pub fn get_connection_pool(config: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new().connect_lazy_with(config.with_db())
}
