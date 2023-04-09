use actix_web::web::{Data, Form};
use actix_web::{HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::Utc;
use sqlx::types::Uuid;
use sqlx::PgPool;

use crate::domain::Subscriber;

#[derive(Debug, Deserialize, Serialize)]
pub struct SubscriberCreateRequest {
    pub email: String,
    pub name: String,
}

#[tracing::instrument(
name = "Adding a new subscriber", skip(subscriber_request, db_pool),
fields(
subscriber_email = % subscriber_request.email,
subscriber_name = % subscriber_request.name
)
)]
pub async fn subscriptions(
    subscriber_request: Form<SubscriberCreateRequest>,
    db_pool: Data<PgPool>,
) -> impl Responder {
    tracing::info!(
        "Adding new subscriber with email: [{}]",
        subscriber_request.email
    );

    let subscriber_to_create = match subscriber_request.0.try_into() {
        Ok(subscriber) => subscriber,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    match create_new_subscriber(&subscriber_to_create, db_pool.get_ref()).await {
        Ok(_) => {
            tracing::info!("Successfully added new subscriber");
            HttpResponse::Ok().finish()
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
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
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES($1, $2, $3, $4)
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
