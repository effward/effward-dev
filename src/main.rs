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

    HttpResponse::Ok().body("Hello Kelly + Jissel! version2")
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

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{
        http::{self, header::ContentType},
        test, web, App,
    };

    #[actix_web::test]
    async fn test_manual_hello_ok() {
        let request = test::TestRequest::default()
            .insert_header(ContentType::plaintext())
            .to_http_request();

        let response = manual_hello().await.respond_to(&request);
        assert_eq!(response.status(), http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_hello_get() {
        let app = test::init_service(App::new().route("/", web::get().to(manual_hello))).await;
        let request = test::TestRequest::default()
            .insert_header(ContentType::plaintext())
            .to_request();

        let response = test::call_service(&app, request).await;
        assert!(response.status().is_success());
    }
}
