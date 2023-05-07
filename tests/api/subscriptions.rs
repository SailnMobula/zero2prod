use sqlx::query;

use crate::helpers::spawn_app;

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

#[tokio::test]
async fn subscribe_returns_a_400_when_fields_are_present_but_empty() {
    // Arrange
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=&email=ursula_le_guin%40gmail.com", "empty name"),
        // ("name=Ursula&email=", "empty email"),
        // ("name=Ursula&email=definitely-not-an-email", "invalid email"),
    ];
    for (body, description) in test_cases {
        // Act
        let response = client
            .post(&format!("http://{}/subscriptions", &test_app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request.");
        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not return a 200 OK when the payload was {}.",
            description
        );
    }
}
