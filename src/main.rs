//! Top-level crate doc
//! ```
//! let x = 5;
//! println!("x is {x}");
//! # assert_eq!(x, 7);
//! ```
mod models;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{http::header, post, web, App, HttpResponse, HttpServer, Responder};
use chrono::prelude::*;
use dotenv::dotenv;
use env_logger;
use hex::ToHex;
use pbkdf2::pbkdf2_hmac_array;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
use std::env;
use std::str;
use tera::{Context, Tera};
use uuid::Uuid;

use crate::models::{EmailModel, UserModel};

#[derive(Serialize)]
struct User {
    id: String,
    name: String,
    created: String,
}

#[derive(Serialize)]
struct Post {
    author: String,
    title: String,
    body: String,
}

#[derive(Debug, Deserialize)]
struct Submission {
    title: String,
    _body: String,
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

pub struct AppState {
    tera: Tera,
    pool: MySqlPool,
}

async fn index(state: web::Data<AppState>) -> impl Responder {
    println!("Home request");

    let tera = &state.tera;
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

async fn signup(state: web::Data<AppState>) -> impl Responder {
    println!("Signup request");

    let tera = &state.tera;
    let mut data = Context::new();
    data.insert("title", "effward.dev - sign up");

    // TODO: use unwrap_or_else() and handle error
    let rendered = tera.render("signup.html", &data).unwrap();

    HttpResponse::Ok().body(rendered)
}

fn hash_password(password: String, salt: &[u8]) -> String {
    const HASH_FUNC: &str = "sha256_1024";
    const SEPARATOR: &str = ":";
    const N: u32 = 1024; // number of iterations

    let raw_password = password.as_bytes();
    let hash = pbkdf2_hmac_array::<Sha256, 20>(raw_password, salt, N);

    let hash_hex = hash.encode_hex::<String>();
    let salt_str = str::from_utf8(salt).unwrap();
    return hash_hex + SEPARATOR + salt_str + SEPARATOR + HASH_FUNC;
}

async fn process_signup(
    data: web::Form<SignupRequest>,
    state: web::Data<AppState>,
) -> impl Responder {
    println!(
        "Received signup request for: {} ({})",
        data.username, data.email
    );

    let pool = &state.pool;

    let salt_uuid = Uuid::new_v4().simple().to_string();
    let salt = salt_uuid[..6].as_bytes();

    let password = hash_password(data.password.to_owned(), salt);

    let created = Utc::now();

    let uuid = Uuid::new_v4();
    let public_id = uuid.into_bytes();

    let email_id = match sqlx::query_as!(
        EmailModel,
        r#"
SELECT *
FROM emails
WHERE address = ?
        "#,
        &data.email.to_lowercase()
    )
    .fetch_one(pool)
    .await
    {
        Ok(email) => email.id,
        Err(_err) => {
            let new_id = sqlx::query!(
                r#"
INSERT INTO emails (address, created)
VALUES (?, ?)
                "#,
                &data.email.to_lowercase(),
                created
            )
            .execute(pool)
            .await
            .unwrap()
            .last_insert_id();

            new_id
        }
    };

    let user_id = sqlx::query!(
        r#"
INSERT INTO users (public_id, name, email_id, password, is_deleted, created, updated)
VALUES (?, ?, ?, ?, ?, ?, ?)
        "#,
        &public_id[..],
        &data.username,
        email_id,
        password,
        0,
        created,
        created
    )
    .execute(pool)
    .await
    .unwrap()
    .last_insert_id();

    HttpResponse::Ok().body(format!("Successfully saved user: {}", user_id))
}

async fn login(state: web::Data<AppState>) -> impl Responder {
    println!("Login request");

    let tera = &state.tera;
    let mut data = Context::new();
    data.insert("title", "effward.dev - log in");

    // TODO: use unwrap_or_else() and handle error
    let rendered = tera.render("login.html", &data).unwrap();

    HttpResponse::Ok().body(rendered)
}

async fn process_login(
    data: web::Form<LoginRequest>,
    state: web::Data<AppState>,
) -> impl Responder {
    println!("Received login request for: {}", data.username);

    let pool = &state.pool;

    let query_result = sqlx::query_as!(
        UserModel,
        r#"
SELECT *
FROM users
WHERE name = ?
        "#,
        &data.username
    )
    .fetch_one(pool)
    .await;

    match query_result {
        Ok(user) => {
            println!("Found User in DB");

            // password verification
            let parts: Vec<&str> = user.password.split(':').collect();
            if parts.len() != 3 {
                return HttpResponse::InternalServerError().json(
                    serde_json::json!({"status": "error","message": "password corrupted in DB"}),
                );
            }
            let salt = parts[1];
            let password = hash_password(data.password.to_owned(), salt.as_bytes());

            if password == user.password {
                let user_response = serde_json::json!({"status": "success", "data": serde_json::json!({
                    "user": translate_user(&user)
                })});

                return HttpResponse::Ok().json(user_response);
            } else {
                return HttpResponse::Unauthorized().json(
                    serde_json::json!({"status": "error", "message": "Incorrect password."}),
                );
            }
        }
        Err(err) => {
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error","message": format!("{:?}", err)}));
        }
    }
}

fn translate_user(user: &UserModel) -> User {
    let mut bytes: [u8; 16] = [0; 16];
    let mut i = 0;
    for byte in &user.public_id {
        bytes[i] = *byte;
        i += 1;

        if i >= 16 {
            break;
        }
    }
    let public_id = Uuid::from_bytes(bytes);
    return User {
        id: public_id.simple().to_string(),
        name: user.name.to_owned(),
        created: user.created.to_string(),
    };
}

async fn submit(state: web::Data<AppState>) -> impl Responder {
    println!("Submit request");

    let tera = &state.tera;
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
    println!("🚀 Starting effward-dev...");

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    dotenv().ok();
    env_logger::init();

    let url =
        env::var("DATABASE_URL").expect("DATABASE_URL environment variable required, but not set");
    let db_server = get_server(&url).unwrap();
    println!("Trying to connect to {}", db_server);
    /* let builder = mysql::OptsBuilder::from_opts(mysql::Opts::from_url(&url).unwrap());
    let pool = mysql::Pool::new(builder.ssl_opts(mysql::SslOpts::default())).unwrap();
    let _connection = pool.get_conn().unwrap();*/

    let pool = match MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&url)
        .await
    {
        Ok(pool) => {
            println!("✅ Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    println!("🚀 Starting HttpServer...");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:8080")
            .allowed_origin("https://*.effward.dev")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();

        let tera = Tera::new("static/templates/**/*").unwrap();
        App::new()
            .app_data(web::Data::new(AppState {
                tera,
                pool: pool.clone(),
            }))
            .route("/", web::get().to(index))
            .route("/signup", web::get().to(signup))
            .route("/signup", web::post().to(process_signup))
            .route("/login", web::get().to(login))
            .route("/login", web::post().to(process_login))
            .route("/submit", web::get().to(submit))
            .route("/submit", web::post().to(process_submission))
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .wrap(cors)
            .wrap(Logger::default())
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

fn get_server(connection: &str) -> Result<String, String> {
    let parts: Vec<&str> = connection.split('@').collect();
    let num_parts = parts.len();
    if num_parts < 2 {
        return Err(format!(
            "Connection string must contain at least one '@' character: {}",
            connection
        ));
    }
    let last_part = parts[num_parts - 1];
    Ok(last_part.to_string())
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
