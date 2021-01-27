use actix_web::{web, Responder, get, HttpResponse, post};

#[get("/environments")]
pub async fn list_environment() -> impl Responder {

    HttpResponse::Ok().finish()
}

#[post("/environments")]
pub async fn insert_environment() -> impl Responder {

    HttpResponse::NoContent().finish()
}