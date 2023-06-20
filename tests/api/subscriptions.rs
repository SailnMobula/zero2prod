use sqlx::query;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

use crate::helpers::spawn_app;

#[tokio::test]
async fn new_subscriber_returns_200() {
    let test_app = spawn_app().await;
    let new_subscriber = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    Mock::given(path("/email"))
        .and(method("post"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&test_app.email_server)
        .await;

    let res = test_app.post_subscription(new_subscriber.into()).await;

    assert!(res.status().is_success());
    assert_eq!(Some(0), res.content_length());
}

#[tokio::test]
async fn new_subscriber_persists_subscriber() {
    let test_app = spawn_app().await;
    let new_subscriber = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    Mock::given(path("/email"))
        .and(method("post"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&test_app.email_server)
        .await;

    let _ = test_app.post_subscription(new_subscriber.into()).await;
    let saved = query!("SELECT email, name, status FROM subscriptions",)
        .fetch_one(&test_app.db_pool)
        .await
        .expect("Could not load from DB");

    assert_eq!(saved.name, "le guin");
    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.status, "pending_confirmation");
}

#[tokio::test]
async fn bad_subscriber_returns_400() {
    let test_app = spawn_app().await;
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (new_subscriber, error_message) in test_cases {
        let res = test_app.post_subscription(new_subscriber.into()).await;
        assert_eq!(
            400,
            res.status().as_u16(),
            "Api did not fail with payload [{}]",
            error_message
        );
    }
}

#[tokio::test]
async fn subscribe_returns_a_400_when_fields_are_present_but_empty() {
    // Arrange
    let test_app = spawn_app().await;
    let test_cases = vec![
        ("name=&email=ursula_le_guin%40gmail.com", "empty name"),
        ("name=Ursula&email=", "empty email"),
        ("name=Ursula&email=definitely-not-an-email", "invalid email"),
    ];
    for (body, description) in test_cases {
        // Act
        let response = test_app.post_subscription(body.into()).await; // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not return a 200 OK when the payload was {}.",
            description
        );
    }
}

#[tokio::test]
async fn subscriber_sends_a_confirmation_email_for_valid_data() {
    let app = spawn_app().await;
    let email = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    Mock::given(path("/email"))
        .and(method("post"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;
    app.post_subscription(email.into()).await;
}

#[tokio::test]
async fn subscribe_sends_a_confirmation_email_with_link() {
    let app = spawn_app().await;
    let email = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    Mock::given(path("/email"))
        .and(method("post"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;

    app.post_subscription(email.into()).await;

    let email_request = &app.email_server.received_requests().await.unwrap()[0];
    let confirmation_links = app.get_confirmation_links(&email_request);

    assert_eq!(confirmation_links.html, confirmation_links.plain);
}
