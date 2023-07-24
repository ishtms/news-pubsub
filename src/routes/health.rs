use actix_web::{get, HttpResponse};
use tracing::{event, Level};

#[get("/health_check")]
#[tracing::instrument(name = "Health Check", fields(span_id = nanoid::nanoid!()))]
pub async fn health_check() -> HttpResponse {
    event!(Level::INFO, "Health check is OK");
    HttpResponse::Ok().finish()
}
