use actix_web::{web, HttpResponse};
use nanoid::nanoid;
use tokio_postgres::Client;
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

#[tracing::instrument(name = "PG_NATIVE_DRIVER", skip(pg_client), fields(span_id = nanoid!()))]
pub async fn fetch_pg_native(
    // form: web::Query<FormData>,
    pg_client: web::Data<Client>,
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
    let result = pg_client
        .query_one(
            "SELECT email, name FROM subscriptions WHERE email=$1",
            &[&form.email],
        )
        .await
        .unwrap();

    let name: String = result.get(0);
    let email: String = result.get(1);

    event!(Level::INFO, "Query executed  {name}, {email}",);

    // if result.len() == 0 {
    //     event!(Level::ERROR, "Failed to execute query: No result found",);
    //     HttpResponse::InternalServerError().finish()
    // } else {
    //     event!(Level::INFO, "Fetched subscriber details");
    //     HttpResponse::Ok().finish()
    // }
    HttpResponse::Ok().finish()
}
