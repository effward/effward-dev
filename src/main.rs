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
use actix_web::http::StatusCode;
use actix_web::middleware::{Compress, ErrorHandlers, Logger};
use actix_web::web::{scope, Data};
use actix_web::{http::header, web, App, HttpServer};
use actix_web_flash_messages::storage::CookieMessageStore;
use actix_web_flash_messages::{FlashMessagesFramework, Level};
use dotenv::dotenv;
use env_logger;
use log::{error, warn};
use sqlx::mysql::MySqlPoolOptions;
use std::env;
use std::str;
use std::sync::Arc;
use tera::Tera;

use crate::entities::user::UserStore;
use crate::routes::{
    comment, error, health, index, login, logout, post, posts, signup, submit, user,
};

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

    let cache = entities::cache::Cache::new();
    let sql_user_store = entities::user::SqlUserStore::new(pool.clone());
    let cached_user_store = entities::user::CachedUserStore::new(cache, sql_user_store);
    let user_store_arc: Arc<dyn UserStore> = Arc::new(cached_user_store);
    let user_store: Data<dyn UserStore> = Data::from(user_store_arc);

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
    let message_framework = FlashMessagesFramework::builder(message_store)
        .minimum_level(Level::Debug)
        .build();
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
            .wrap(
                ErrorHandlers::new()
                    .default_handler(error::generic::get::render_generic)
                    .handler(
                        StatusCode::NOT_FOUND,
                        error::not_found::get::render_not_found,
                    ),
            )
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
            .route("/comment", web::post().to(comment::post::process_comment))
            .route("/submit", web::get().to(submit::get::submit))
            .route("/submit", web::post().to(submit::post::process_submission))
            .route("/user/{user}", web::get().to(user::get::user))
            .route("/post/{post}", web::get().to(post::get::post))
            .route("/posts", web::get().to(posts::get::posts))
            .route("/health", web::get().to(health::get::health))
            .service(
                scope("/error")
                    .route("/404", web::get().to(error::not_found::get::not_found))
                    .route("/generic", web::get().to(error::generic::get::generic)),
            )
            .service(Files::new("/static", "public").show_files_listing())
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(tera.clone()))
            .app_data(user_store.clone())
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
