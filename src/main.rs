//! Top-level crate doc
//! ```
//! let x = 5;
//! println!("x is {x}");
//! # assert_eq!(x, 7);
//! ```
mod entities;
mod routes;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{http::header, web, App, HttpServer};
use dotenv::dotenv;
use env_logger;
use sqlx::mysql::MySqlPoolOptions;
use std::env;
use std::str;
use tera::Tera;

use crate::routes::{health, home, login, signup, submit};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Starting effward-dev...");

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "info"); // set to "debug" for more logs
    }
    dotenv().ok();
    env_logger::init();

    let url =
        env::var("DATABASE_URL").expect("DATABASE_URL environment variable required, but not set");
    let db_server = get_server(&url).unwrap();
    println!("Trying to connect to {}", db_server);

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
            .allowed_origin("https://effward.dev")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();

        let tera = Tera::new("static/templates/**/*").unwrap();
        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .route("/", web::get().to(home::get::home))
            .route("/signup", web::get().to(signup::get::signup))
            .route("/signup", web::post().to(signup::post::process_signup))
            .route("/login", web::get().to(login::get::login))
            .route("/login", web::post().to(login::post::process_login))
            .route("/submit", web::get().to(submit::get::submit))
            .route("/submit", web::post().to(submit::post::process_submission))
            .route("/health", web::get().to(health::get::health))
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(tera))
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
