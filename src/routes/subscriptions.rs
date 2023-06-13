use actix_web::web::{Data, Form};
use actix_web::{HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::Utc;
use sqlx::types::Uuid;
use sqlx::PgPool;

use crate::domain::Subscriber;
use crate::email_client::EmailClient;
use crate::startup::ApplicationBaseUrl;

#[derive(Debug, Deserialize, Serialize)]
pub struct SubscriberCreateRequest {
    pub email: String,
    pub name: String,
}

#[tracing::instrument(
name = "Adding a new subscriber", skip(subscriber_request, db_pool, base_url ),
fields(
subscriber_email = % subscriber_request.email,
subscriber_name = % subscriber_request.name
)
)]
pub async fn subscriptions(
    subscriber_request: Form<SubscriberCreateRequest>,
    db_pool: Data<PgPool>,
    email_client: Data<EmailClient>,
    base_url: Data<ApplicationBaseUrl>,
) -> impl Responder {
    tracing::info!(
        "Adding new subscriber with email: [{}]",
        subscriber_request.email
    );

    let subscriber_to_create = match subscriber_request.0.try_into() {
        Ok(subscriber) => subscriber,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    let token = "mytoken";
    if create_new_subscriber(&subscriber_to_create, db_pool.get_ref())
        .await
        .is_err()
    {
        return HttpResponse::InternalServerError().finish();
    }

    tracing::info!("Sending confirmation mail to new subscriber");

    if send_email_confirmation(&email_client, &base_url.0, subscriber_to_create, token)
        .await
        .is_err()
    {
        return HttpResponse::InternalServerError().finish();
    }

    tracing::info!("Successfully added new subscriber");
    HttpResponse::Ok().finish()
}

#[tracing::instrument(
    name = "Send a confirmation email to new subscriber",
    skip(email_client, subscriber_to_create, base_url)
)]
async fn send_email_confirmation(
    email_client: &EmailClient,
    base_url: &str,
    subscriber_to_create: Subscriber,
    subscription_token: &str,
) -> Result<(), reqwest::Error> {
    println!("base: {}", base_url);
    let confirmation_link = format!(
        "{}/subscriptions/confirm?subscription_token={}",
        base_url, subscription_token
    );
    let html_body = format!(
        "Welcome to our newsletter! <br /> \
                Please click <a href=\"{}\">here</a> to confirm your registration",
        confirmation_link
    );
    let plain_body = format!(
        "Welcome to our newsletter!\n Visit {} to confirm your registration",
        confirmation_link
    );

    email_client
        .send_mail(
            subscriber_to_create.email,
            "Welcome",
            &html_body,
            &plain_body,
        )
        .await
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(subscriber, db_pool)
)]
async fn create_new_subscriber(
    subscriber: &Subscriber,
    db_pool: &PgPool,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at, status)
        VALUES($1, $2, $3, $4, 'pending_confirmation') 
        "#,
        Uuid::new_v4(),
        subscriber.email.as_ref(),
        subscriber.name.as_ref(),
        Utc::now()
    )
    .execute(db_pool)
    .await
    .map_err(|e| {
        tracing::info!("Failed to add subscriber cause: [{:?}]", e);
        e
    })?;
    Ok(())
}
