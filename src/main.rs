//! Top-level crate doc
//! ```
//! let x = 5;
//! println!("x is {x}");
//! # assert_eq!(x, 7);
//! ```
mod entities;
mod routes;

use actix_cors::Cors;
use actix_files::Files;
use actix_session::storage::RedisSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::Key;
use actix_web::middleware::{Logger, Compress};
use actix_web::{http::header, web, App, HttpServer};
use actix_web_flash_messages::storage::CookieMessageStore;
use actix_web_flash_messages::FlashMessagesFramework;
use dotenv::dotenv;
use env_logger;
use log::{error, warn};
use sqlx::mysql::MySqlPoolOptions;
use std::env;
use std::str;
use tera::Tera;

use crate::routes::{health, index, login, logout, post, posts, signup, submit, users};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("📜 Setting up env_logger...");
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "info"); // set to "debug" for more logs, "warn" for fewer
    }
    dotenv().ok();
    env_logger::init();
    warn!("📜✅ Logger started");

    warn!("🖕 Starting effward-dev dependencies...");

    let url = env::var("DATABASE_URL")
        .expect("📚🔥 DATABASE_URL environment variable required, but not set");
    let db_server = get_server(&url).unwrap();

    warn!("📚 Connecting to MySQL DB: {}", db_server);
    let pool = match MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&url)
        .await
    {
        Ok(pool) => {
            warn!("📚✅ Connection to the database is successful!");
            pool
        }
        Err(err) => {
            error!("📚🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    warn!("🌎 Initializing Tera static templates...");
    let tera = match Tera::new("templates/**/*") {
        Ok(t) => {
            warn!("🌎✅ Static templates initialized.");
            t
        }
        Err(e) => {
            error!("🌎🔥 Failed to initialize Tera: {:?}", e);
            std::process::exit(1);
        }
    };

    warn!("💡 Initializing Flash Message Framework...");
    let hmac_key = match env::var("HMAC_KEY") {
        Ok(key) => key,
        Err(e) => {
            error!(
                "💡🔥 HMAC_KEY environment variable is not set! Error: {}",
                e
            );
            std::process::exit(1);
        }
    };
    let secret_key = Key::from(hmac_key.as_bytes());
    let message_store = CookieMessageStore::builder(secret_key.clone()).build();
    let message_framework = FlashMessagesFramework::builder(message_store).build();
    warn!("💡✅ Flash Message Framework initialized.");

    warn!("🗝️ Connecting to redis...");
    let reds_uri = match env::var("REDIS_URI") {
        Ok(uri) => uri,
        Err(e) => {
            error!(
                "🗝️🔥 REDIS_URI environment variable is not set! Error: {}",
                e
            );
            std::process::exit(1);
        }
    };
    let redis_store = match RedisSessionStore::new(reds_uri).await {
        Ok(store) => store,
        Err(e) => {
            error!("🗝️🔥 Error creating RedisSessionStore. Error: {}", e);
            std::process::exit(1);
        }
    };
    warn!("🗝️✅ Connected to redis.");

    warn!("🖕✅ Finished starting effward-dev dependencies.");
    warn!("🚀 Starting HttpServer...");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:8080")
            .allowed_origin("https://*.effward.dev")
            .allowed_origin("https://effward.dev")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();

        App::new()
            .wrap(Compress::default())
            .wrap(cors)
            .wrap(message_framework.clone())
            .wrap(SessionMiddleware::new(
                redis_store.clone(),
                secret_key.clone(),
            ))
            .wrap(Logger::default())
            .route("/", web::get().to(index::get::index))
            .route("/signup", web::get().to(signup::get::signup))
            .route("/signup", web::post().to(signup::post::process_signup))
            .route("/login", web::get().to(login::get::login))
            .route("/login", web::post().to(login::post::process_login))
            .route("/logout", web::post().to(logout::post::process_logout))
            .route("/submit", web::get().to(submit::get::submit))
            .route("/submit", web::post().to(submit::post::process_submission))
            .route("/users/{user}", web::get().to(users::get::users))
            .route("/post/{post}", web::get().to(post::get::post))
            .route("/posts", web::get().to(posts::get::posts))
            .route("/health", web::get().to(health::get::health))
            .service(Files::new("/static", "public").show_files_listing())
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(tera.clone()))
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
