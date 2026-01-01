use actix_web::{get, post, web, App, Result, HttpResponse, HttpServer, Responder};
use actix_files::NamedFile;
use std::path::PathBuf;

#[get("/")]
async fn index() -> Result<NamedFile> {
    Ok(NamedFile::open("static/index.html")?)
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {   
  HttpServer::new(|| {
        App::new()
            .service(index)
            .service(echo)
            .route("/hey", web::get().to( manual_hello))
        })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}

