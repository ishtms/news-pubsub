use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use tracing::{event, Level};
use uuid::Uuid;

#[derive(Debug, serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

#[tracing::instrument(name = "Subscribe to newsletter", skip(db_connection))]
pub async fn subscribe(
    form: web::Form<FormData>,
    db_connection: web::Data<PgPool>,
) -> HttpResponse {
    event!(
        Level::INFO,
        "Saving a new subscriber - {}, {}",
        form.name,
        form.email
    );

    let result = sqlx::query!(
        r#"
    INSERT INTO subscriptions(id, email, name, subscribed_at)
    VALUES($1, $2, $3, $4);
    "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(db_connection.get_ref())
    .await;

    if result.is_err() {
        event!(
            Level::ERROR,
            "Failed to execute query: {}",
            result.err().unwrap().to_string()
        );
        HttpResponse::InternalServerError().finish()
    } else {
        event!(Level::INFO, "New subscriber details have been saved");
        HttpResponse::Ok().finish()
    }
}
