use actix_files as fs;
use actix_web::{App, HttpServer};
mod routes;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(routes::index)
    })
    .bind("127.0.0.1:9000")?
    .run()
    .await
}
