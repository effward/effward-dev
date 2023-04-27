//! Top-level crate doc
//! ```
//! let x = 5;
//! println!("x is {x}");
//! # assert_eq!(x, 7);
//! ```
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    println!("Home request");

    HttpResponse::Ok().body("Hello Kelly + Jissel!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    println!("Echo request");

    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    println!("Hey request");

    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting effward-dev...");

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}