use actix_web::{HttpServer, Responder, App, HttpResponse};
use actix_web::get;

#[get("/hello")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            // .route("/hello", web::get().to(index))
            // .route("/hello-again", web::get().to(index2))
    })
    .bind("127.0.0.1:9000")?
    .run()
    .await
} 
