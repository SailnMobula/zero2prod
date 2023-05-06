use std::net::TcpListener;

use sqlx::PgPool;

use zero2prod::configuration::get_configuration;
use zero2prod::email_client::EmailClient;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    logs::info!("Load config");
    let config = get_configuration().expect("Could not read configuration");
    logs::info!("Config loaded");
    let address = format!("{}:{}", config.application.host, config.application.port);
    logs::info!("bind port {}", address);
    let connection_string = config.database.get_connection_string();
    logs::info!("Connect to database");
    let db_pool = PgPool::connect_lazy(&connection_string).expect("Could not connect to Database");
    logs::info!("Successfully connected to database");
    let email = config
        .email_client
        .sender()
        .expect("Not a valid email address");
    let email_client = EmailClient::new(
        config.email_client.base_url,
        email,
        config.email_client.auth_token,
    );
    let listener = TcpListener::bind(address).expect("Failed to bind port");
    logs::info!("Starting app");
    run(listener, db_pool, email_client)?.await
}
