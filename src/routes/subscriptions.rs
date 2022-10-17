use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    // `Result` has two variants: `Ok` and `Err`.
    // The first for success, and the second for failures
    // We use a `match` statement to choose what to do based
    // on the outcome.
    // We will talk more about `Result` going forward!
    let correlation_id = Uuid::new_v4();
    log::info!(
        "correlation_id: {} - Adding '{}' '{}' as a new subscriber.",
        correlation_id,
        form.email,
        form.name
    );
    log::info!(
        "correlation_id: {} - Saving new subscriber details in database.",
        correlation_id,
        form.email,
        form.name
    );
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        values ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    // We use `get_ref` to get an immutable reference to the `PgConnection`
    // wrapped by `web::Data`
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => {
            log::info!(
                "correlation_id: {} - New subscriber details have been saved",
                correlation_id
            );
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            log::error!(
                "correlation_id: {} - Failed to execute query: {:?}",
                correlation_id,
                e
            );
            HttpResponse::InternalServerError().finish()
        }
    }
}
