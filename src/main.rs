use std::net::TcpListener;

use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = get_configuration().expect("Could not read configuration");
    let address = format!("127.0.0.1:{}", config.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind port");
    run(listener)?.await
}
