use std::net::TcpListener;

use once_cell::sync::Lazy;
use sqlx::{query, PgPool};

use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "zero2prod".to_string();

    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    };
});

#[tokio::test]
async fn health_check_returns_ok() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    let res = client
        .get(&format!("http://{}/health_check", &test_app.address))
        .send()
        .await
        .expect("Failed to send request");

    assert!(res.status().is_success());
    assert_eq!(Some(0), res.content_length());
}

#[tokio::test]
async fn new_subscriber_returns_200() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    let new_subscriber = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let res = client
        .post(&format!("http://{}/subscriptions", test_app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(new_subscriber)
        .send()
        .await
        .expect("Failed to send request");

    let saved = query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&test_app.db_pool)
        .await
        .expect("Could not load from DB");

    assert!(res.status().is_success());
    assert_eq!(Some(0), res.content_length());
    assert_eq!("le guin", saved.name);
    assert_eq!("ursula_le_guin@gmail.com", saved.email);
}

#[tokio::test]
async fn bad_subscriber_returns_400() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (new_subscriber, error_message) in test_cases {
        let res = client
            .post(&format!("http://{}/subscriptions", test_app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(new_subscriber)
            .send()
            .await
            .expect("Failed to send request");

        assert_eq!(
            400,
            res.status().as_u16(),
            "Api did not fail with payload [{}]",
            error_message
        );
    }
}

async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);

    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind address");
    let address = listener.local_addr().expect("No address");
    let config = get_configuration().expect("Could not read configuration");
    let connection_string = config.database.get_connection_string();
    let db_pool = PgPool::connect(&connection_string)
        .await
        .expect("Could not connect to Database");
    let server = run(listener, db_pool.clone()).expect("Failed");

    let _ = tokio::spawn(server);

    let address = format!("{}", address);

    TestApp { address, db_pool }
}

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}
