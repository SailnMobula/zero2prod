use crate::helpers::spawn_app;

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
