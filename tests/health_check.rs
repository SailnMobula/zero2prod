use std::net::TcpListener;

use sqlx::{query, Connection, PgConnection};

use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::test]
async fn health_check_returns_ok() {
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    let res = client
        .get(&format!("http://{}/health_check", &app_address))
        .send()
        .await
        .expect("Failed to send request");

    assert!(res.status().is_success());
    assert_eq!(Some(0), res.content_length());
}

#[tokio::test]
async fn new_subscriber_returns_200() {
    let app_address = spawn_app();
    let config = get_configuration().expect("Could not read configuration");
    let connection_string = config.database.get_connection_string();
    let mut connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to database");
    let client = reqwest::Client::new();

    let new_subscriber = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let res = client
        .post(&format!("http://{}/subscriptions", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(new_subscriber)
        .send()
        .await
        .expect("Failed to send request");

    let saved = query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&mut connection)
        .await
        .expect("Could not load from DB");

    assert!(res.status().is_success());
    assert_eq!(Some(0), res.content_length());
    assert_eq!("le guin", saved.name);
    assert_eq!("ursula_le_guin@gmail.com", saved.email);
}

#[tokio::test]
async fn bad_subscriber_returns_400() {
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (new_subscriber, error_message) in test_cases {
        let res = client
            .post(&format!("http://{}/subscriptions", &app_address))
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

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind address");
    let address = listener.local_addr().expect("No address");
    let server = run(listener).expect("Failed");

    let _ = tokio::spawn(server);

    format!("{}", address)
}
