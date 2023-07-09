use actix_cors::Cors;
use actix_files::Files;
use actix_session::SessionMiddleware;
use actix_web::cookie::Key;
use actix_web::dev::Server;
use actix_web::http::{header, StatusCode};
use actix_web::middleware::{Compress, ErrorHandlers, Logger};
use actix_web::web::{self, scope};
use actix_web::{App, HttpServer};
use dotenv::dotenv;
use log::warn;

use crate::entities::EntityStores;
use crate::routes::{
    comment, error, health, index, login, logout, post, posts, signup, submit, user,
};
use crate::server::{
    db::init_db, environment::Environment, flash_messages::init_flash_messages, redis::init_redis,
    session::init_session_store, tera::init_tera,
};

use super::ServerError;

pub struct Application {
    server: Server,
}

impl Application {
    pub async fn new(
        env: Environment,
        port: u16,
        log_level: &str,
        db_url: &str,
        redis_uri: &str,
        hmac_key: &str,
    ) -> Result<Self, ServerError> {
        let secret_key = Key::from(hmac_key.as_bytes());

        init_logger(log_level);

        warn!("🖕 Starting effward-dev dependencies...");
        let db_pool = init_db(db_url).await?;
        let _redis_client = init_redis(redis_uri).await?;
        let tera = init_tera()?;
        let flash_messages = init_flash_messages(secret_key.clone());
        let session_store = init_session_store(redis_uri).await?;
        let entity_stores = EntityStores::new(db_pool.clone());
        warn!("🖕 Finished starting effward-dev dependencies.");

        warn!("🚀 Starting HttpServer...");
        let server = HttpServer::new(move || {
            let mut cors = Cors::default()
                .allowed_origin("https://*.effward.dev")
                .allowed_origin("https://effward.dev")
                .allowed_methods(vec!["GET", "POST"])
                .allowed_headers(vec![
                    header::CONTENT_TYPE,
                    header::AUTHORIZATION,
                    header::ACCEPT,
                ])
                .supports_credentials();

            if matches!(env, Environment::Development) {
                cors = cors.allowed_origin("http://localhost:8080")
            }

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
                .wrap(flash_messages.clone())
                .wrap(SessionMiddleware::new(
                    session_store.clone(),
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
                .app_data(web::Data::new(db_pool.clone()))
                .app_data(web::Data::new(tera.clone()))
                .app_data(web::Data::new(entity_stores.clone()))
        })
        .bind(("0.0.0.0", port))?
        .run();

        warn!("🚀 Started HttpServer!");
        Ok(Self { server })
    }

    pub async fn run(self) -> Result<(), ServerError> {
        match self.server.await {
            Ok(()) => Ok(()),
            Err(e) => Err(ServerError::HttpRuntime(e.to_string())),
        }
    }
}

fn init_logger(log_level: &str) {
    println!("📜 Setting up env_logger...");
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", log_level);
    }
    dotenv().ok();
    env_logger::init();
}
