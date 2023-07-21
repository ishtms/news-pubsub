use actix_web::{get, HttpResponse};
use std::fmt;
use tracing::{event, Level};

#[derive(Debug)]
struct Testing {
    name: String,
    value: i32,
}

impl fmt::Display for Testing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Name: {}, Value: {}", self.name, self.value)
    }
}

#[get("/health_check")]
#[tracing::instrument(name = "Subscribe to newsletter")]
pub async fn health_check() -> HttpResponse {
    event!(Level::INFO, "Health Check first log");
    event!(
        Level::INFO,
        "Second log, this one is longer than teh expected {}",
        Testing {
            name: "Hi my name is romero".to_owned(),
            value: 33
        }
    );
    HttpResponse::Ok().finish()
}
