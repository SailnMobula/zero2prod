use actix_web::{web::Query, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Parameters {
    subscription_token: String,
}

#[tracing::instrument(name = "Confirm a subscription", skip(_params))]
pub async fn subscription_confirm(_params: Query<Parameters>) -> HttpResponse {
    println!("{}", _params.subscription_token);
    HttpResponse::Ok().finish()
}
