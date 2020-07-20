use actix_web::get;
use actix_web::{HttpResponse, Responder, Result};
use actix_files::NamedFile;

#[get("/hello")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/")]
pub async fn serve_index_html() -> Result<NamedFile> {
    Ok(NamedFile::open("./src/client/index.html")?)
}
