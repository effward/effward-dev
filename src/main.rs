//! Top-level crate doc
//! ```
//! let x = 5;
//! println!("x is {x}");
//! # assert_eq!(x, 7);
//! ```
use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use tera::{Tera, Context};

async fn index(tera: web::Data<Tera>) -> impl Responder {
    println!("Home request");

    let mut data = Context::new();
    data.insert("title", "effward.dev");
    data.insert("name", "effward");

    // TODO: use unwrap_or_else() and handle error
    let rendered = tera.render("index.html", &data).unwrap();

    HttpResponse::Ok().body(rendered)
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
        let tera = Tera::new("src/templates/**/*").unwrap();
        App::new()
            .app_data(web::Data::new(tera))
            .route("/", web::get().to(index))
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
