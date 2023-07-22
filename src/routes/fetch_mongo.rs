use actix_web::{web, HttpResponse};
use nanoid::nanoid;
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

#[tracing::instrument(name = "MONGO", skip(mongo_connection), fields(span_id = nanoid!()))]
pub async fn fetch_mongo(
    // form: web::Query<FormData>,
    mongo_connection: web::Data<mongodb::Client>,
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
    let result = mongo_connection
        .database("db")
        .collection::<DummyType>("users")
        .find_one(mongodb::bson::doc! { "email": form.email}, None)
        .await
        .unwrap();

    event!(Level::INFO, "Query executed {:?}", result.as_ref().unwrap());

    if result.is_none() {
        event!(Level::ERROR, "Failed to execute query: No result found",);
        HttpResponse::InternalServerError().finish()
    } else {
        event!(Level::INFO, "Fetched subscriber details");
        HttpResponse::Ok().finish()
    }
}
