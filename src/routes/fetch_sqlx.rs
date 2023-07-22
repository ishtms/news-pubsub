use actix_web::{web, HttpResponse};
use nanoid::nanoid;
use sqlx::PgPool;
use tracing::{event, Level};

#[derive(Debug, serde::Deserialize)]
struct FormData {
    email: String,
    name: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct DummyType {
    name: String,
    email: String,
}

#[tracing::instrument(name = "SQLX", skip(db_connection), fields(span_id = nanoid!()))]
pub async fn fetch_sqlx(
    // form: web::Query<FormData>,
    db_connection: web::Data<PgPool>,
) -> HttpResponse {
    let form = FormData {
        email: "ishtmeet".to_owned(),
        name: "smeeoijdf@Lkj".to_owned(),
    };

    event!(
        Level::INFO,
        "Getting a new subscriber - {}, {}",
        form.name,
        form.email
    );

    let result = sqlx::query!(
        r#"
        SELECT email, name
        FROM subscriptions
        WHERE email = $1
        LIMIT 1
        "#,
        form.email
    )
    .fetch_one(db_connection.get_ref())
    .await;

    event!(Level::INFO, "Query executed {:?}", result.as_ref().unwrap());

    if result.is_err() {
        event!(Level::ERROR, "Failed to execute query: No result found",);
        HttpResponse::InternalServerError().finish()
    } else {
        event!(Level::INFO, "Fetched subscriber details");
        HttpResponse::Ok().finish()
    }
}
