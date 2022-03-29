use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use std::io;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[get("/users/{name}")]
async fn echo(name: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello {name}!"))
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(move || App::new().service(index).service(echo))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
