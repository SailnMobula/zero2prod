use zero2prod::configuration::get_configuration;
use zero2prod::startup::Application;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    logs::info!("Load config");
    let config = get_configuration().expect("Could not read configuration");
    let server = Application::build(config).await?;
    server.run_until_stopped().await?;
    Ok(())
}
