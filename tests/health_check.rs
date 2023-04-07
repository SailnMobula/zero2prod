use std::net::TcpListener;

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

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind address");
    let address = listener.local_addr().expect("No address");
    let server = zero2prod::run(listener).expect("Failed");

    let _ = tokio::spawn(server);

    format!("{}", address)
}
