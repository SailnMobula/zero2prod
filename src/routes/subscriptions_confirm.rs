use actix_web::{
    web::{Data, Query},
    HttpResponse,
};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct Parameters {
    subscription_token: String,
}

#[tracing::instrument(name = "Confirm a subscription", skip(params))]
pub async fn subscription_confirm(
    params: Query<Parameters>,
    db_pool: Data<PgPool>,
) -> HttpResponse {
    println!("{}", params.subscription_token);
    let token = params.subscription_token.as_str();
    let subscriber_id = match get_subscriber_id_from_token(token, db_pool.as_ref()).await {
        Ok(query_res) => query_res,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };
    match subscriber_id {
        None => return HttpResponse::Unauthorized().finish(),
        Some(id) => {
            if confirm_subscriber(id, db_pool.as_ref()).await.is_err() {
                return HttpResponse::InternalServerError().finish();
            }
        }
    }
    HttpResponse::Ok().finish()
}

#[tracing::instrument(
    name = "Mark subscriber as confirmed"
    skip(id, db_pool)
)]
async fn confirm_subscriber(id: Uuid, db_pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"UPDATE subscriptions SET status = 'confirmed' WHERE id = $1"#,
        id
    )
    .execute(db_pool)
    .await
    .map_err(|e| {
        tracing::info!("Failed to confirm subscriber [{:?}]", e);
        e
    })?;
    Ok(())
}

#[tracing::instrument(
    name = "Get subscriber from token"
    skip(token, db_pool)
)]
async fn get_subscriber_id_from_token(
    token: &str,
    db_pool: &PgPool,
) -> Result<Option<Uuid>, sqlx::Error> {
    let query_result = sqlx::query!(
        "SELECT subscriber_id FROM subscription_tokens WHERE subscription_token = $1",
        token
    )
    .fetch_optional(db_pool)
    .await
    .map_err(|e| {
        tracing::info!("could not load subscriber [{:?}]", e);
        e
    })?;
    Ok(query_result.map(|r| r.subscriber_id))
}
