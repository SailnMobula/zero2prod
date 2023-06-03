use crate::helpers::spawn_app;

#[tokio::test]
async fn confirmation_without_a_token_rejected_with_400() {
    let app = spawn_app().await;
    let url = format!("http://{}/subscriptions/confirm", &app.address);
    println!("{}", url);
    
    let res = reqwest::get(url)
        .await
        .unwrap();
    assert_eq!(res.status().as_u16(), 400);
}
