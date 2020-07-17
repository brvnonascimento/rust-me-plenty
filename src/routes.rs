use actix_web::get;
use actix_web::{HttpResponse, Responder};

#[get("/hello")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
