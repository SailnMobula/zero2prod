use actix_web::web::{Data, Form};
use actix_web::{HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::Utc;
use sqlx::types::Uuid;
use sqlx::PgPool;

#[derive(Debug, Deserialize, Serialize)]
pub struct Subscriber {
    email: String,
    name: String,
}

#[tracing::instrument(
name = "Adding a new subscriber", skip(subscriber, db_pool),
fields(
subscriber_email = % subscriber.email,
subscriber_name = % subscriber.name
)
)]
pub async fn subscriptions(subscriber: Form<Subscriber>, db_pool: Data<PgPool>) -> impl Responder {
    tracing::info!("Adding new subscriber with email: [{}]", subscriber.email);
    match create_new_subscriber(subscriber, db_pool).await {
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
    subscriber: Form<Subscriber>,
    db_pool: Data<PgPool>,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        subscriber.email,
        subscriber.name,
        Utc::now()
    )
    .execute(db_pool.get_ref())
    .await
    .map_err(|e| {
        tracing::info!("Failed to add subscriber cause: [{:?}]", e);
        e
    })?;
    Ok(())
}
