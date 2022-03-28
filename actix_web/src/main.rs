use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;

// #[get("/")]
// async fn hello() -> impl Responder {
//     HttpResponse::Ok().body("Hello World!")
// }

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

// async fn manual_hello() -> impl Responder {
//     HttpResponse::Ok().body("Hey there!")
// }

// async fn index() -> impl Responder {
//     "Index"
// }

struct AppState {
    app_name: String,
}

struct AppStateWithCounter {
    counter: Mutex<i32>,
}

// #[get("/")]
async fn index(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    // let app_name = &data.app_name;
    // format!("Hello {app_name}!")
    format!("Request number: {counter}")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(counter.clone())
            .route("/", web::get().to(index))
        // App::new().service(web::scope("/app").route("index.html", web::get().to(index)))
        // .service(hello)
        // .service(echo)
        // .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
