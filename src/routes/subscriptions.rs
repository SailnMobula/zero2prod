use actix_web::web::Form;
use actix_web::{HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Subscriber {
    email: String,
    name: String,
}

pub async fn subscriptions(_: Form<Subscriber>) -> impl Responder {
    HttpResponse::Ok()
}
