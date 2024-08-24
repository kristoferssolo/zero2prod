use zero2prod::{
    config::get_config,
    startup::Application,
    telemetry::{get_subscriber, init_subscriber},
};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("zero2prod", "info", std::io::stdout);
    init_subscriber(subscriber);
    let config = get_config().expect("Failed to read configuation.");
    let application = Application::build(config.clone()).await?;
    application.run_until_stopped().await?;
    Ok(())
}
