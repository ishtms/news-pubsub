use actix_web::{web, HttpResponse};
#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    println!("Received Email: {} Name: {}", _form.email, _form.name);
    HttpResponse::Ok().finish()
}
