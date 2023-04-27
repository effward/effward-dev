//! Top-level crate doc
//! ```
//! let x = 5;
//! println!("x is {x}");
//! # assert_eq!(x, 7);
//! ```
use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use tera::{Context, Tera};

#[derive(Serialize)]
struct Post {
    author: String,
    title: String,
    body: String,
}

#[derive(Debug, Deserialize)]
struct Submission {
    title: String,
    body: String,
}

#[derive(Debug, Deserialize)]
struct SignupRequest {
    username: String,
    email: String,
    password: String,
}

#[derive(Debug, Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

async fn index(tera: web::Data<Tera>) -> impl Responder {
    println!("Home request");

    let mut data = Context::new();

    let posts = [
        Post {
            title: String::from("First Post"),
            body: String::from("Body content"),
            author: String::from("effward"),
        },
        Post {
            title: String::from("Second Post"),
            body: String::from("Second body content!"),
            author: String::from("beeward"),
        },
        Post {
            title: String::from("Third Post"),
            body: String::from("Third body content?"),
            author: String::from("effward"),
        },
    ];

    data.insert("title", "effward.dev - home");
    data.insert("name", "effward");
    data.insert("posts", &posts);

    // TODO: use unwrap_or_else() and handle error
    let rendered = tera.render("index.html", &data).unwrap();

    HttpResponse::Ok().body(rendered)
}

async fn signup(tera: web::Data<Tera>) -> impl Responder {
    println!("Signup request");

    let mut data = Context::new();
    data.insert("title", "effward.dev - sign up");

    // TODO: use unwrap_or_else() and handle error
    let rendered = tera.render("signup.html", &data).unwrap();

    HttpResponse::Ok().body(rendered)
}

async fn process_signup(data: web::Form<SignupRequest>) -> impl Responder {
    println!(
        "Received signup request for: {} ({})",
        data.username, data.email
    );

    HttpResponse::Ok().body(format!("Successfully saved user: {}", data.username))
}

async fn login(tera: web::Data<Tera>) -> impl Responder {
    println!("Login request");

    let mut data = Context::new();
    data.insert("title", "effward.dev - log in");

    // TODO: use unwrap_or_else() and handle error
    let rendered = tera.render("login.html", &data).unwrap();

    HttpResponse::Ok().body(rendered)
}

async fn process_login(data: web::Form<LoginRequest>) -> impl Responder {
    println!("Received login request for: {}", data.username);

    HttpResponse::Ok().body(format!(
        "Successfully received login for: {}",
        data.username
    ))
}

async fn submit(tera: web::Data<Tera>) -> impl Responder {
    let mut data = Context::new();
    data.insert("title", "effward.dev - submit");

    let rendered = tera.render("submit.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}

async fn process_submission(data: web::Form<Submission>) -> impl Responder {
    println!("{:?}", data);
    HttpResponse::Ok().body(format!("Posted submission: {}", data.title))
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
            .route("/signup", web::get().to(signup))
            .route("/signup", web::post().to(process_signup))
            .route("/login", web::get().to(login))
            .route("/login", web::post().to(process_login))
            .route("/submit", web::get().to(submit))
            .route("/submit", web::post().to(process_submission))
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
