use std::io::Error;
use std::net::TcpListener;
use std::time::Duration;

use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tracing_actix_web::TracingLogger;

use crate::configuration::{DatabaseSettings, Settings};
use crate::email_client::EmailClient;
use crate::routes::{health_check, subscription_confirm, subscriptions};

pub struct Application {
    server: Server,
    port: u16,
}

pub struct ApplicationBaseUrl(pub String);

pub fn get_connection_pool(config: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new()
        .acquire_timeout(Duration::from_secs(2))
        .connect_lazy_with(config.with_db())
}

impl Application {
    pub async fn build(config: Settings) -> Result<Self, Error> {
        let timeout = config.email_client.timeout();
        logs::info!("Config loaded");
        let address = format!("{}:{}", config.application.host, config.application.port);
        logs::info!("bind port {}", address);
        let db_pool = get_connection_pool(&config.database);
        let email = config
            .email_client
            .sender()
            .expect("Not a valid email address");
        let email_client = EmailClient::new(
            config.email_client.base_url,
            email,
            config.email_client.auth_token,
            timeout,
        );
        let base_url = config.application.base_url;
        let listener = TcpListener::bind(&address).expect("Failed to bind port");
        let port = listener.local_addr().unwrap().port();
        let server =
            Self::run(listener, db_pool, email_client, base_url).expect("Failed to run app");
        Ok(Application { port, server })
    }
    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), Error> {
        self.server.await
    }

    pub fn run(
        listener: TcpListener,
        dp_pool: PgPool,
        email_client: EmailClient,
        base_url: String,
    ) -> Result<Server, std::io::Error> {
        let db_pool = Data::new(dp_pool);
        let email_client = Data::new(email_client);
        let base_url = Data::new(ApplicationBaseUrl(base_url));
        let server = HttpServer::new(move || {
            App::new()
                .wrap(TracingLogger::default())
                .route("/health_check", web::get().to(health_check))
                .route("/subscriptions", web::post().to(subscriptions))
                .route(
                    "/subscriptions/confirm",
                    web::get().to(subscription_confirm),
                )
                .app_data(db_pool.clone())
                .app_data(email_client.clone())
                .app_data(base_url.clone())
        })
        .listen(listener)?
        .run();

        Ok(server)
    }
}
