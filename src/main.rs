use actix_web::{App, HttpServer};
mod routes;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(routes::index)
        // App::new()
        //     .handler(
        //         '/',
        //     )
        // .route("/hello", web::get().to(index))
        // .route("/hello-again", web::get().to(index2))
    })
    .bind("127.0.0.1:9000")?
    .run()
    .await
}
