//! Top-level crate doc
//! ```
//! let x = 5;
//! println!("x is {x}");
//! # assert_eq!(x, 7);
//! ```

// TODO: ^^^ Update ^^^

use std::{env, str::FromStr};

use effward_dev::server::{Application, Environment, ServerError};

#[actix_web::main]
async fn main() -> Result<(), ServerError> {
    let env = match env::var("EFFWARD_DEV_ENVIRONMENT") {
        Ok(e) => Environment::from_str(&e)?,
        Err(err) => {
            println!("🔥 EFFWARD_DEV_ENVIRONMENT not set: {}", err);
            std::process::exit(1);
        }
    };

    let db_url = match env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(e) => {
            println!(
                "🔥 DATABASE_URL environment variable required, but not set: {}",
                e
            );
            std::process::exit(1);
        }
    };

    let hmac_key = match env::var("HMAC_KEY") {
        Ok(key) => key,
        Err(e) => {
            println!("🔥 HMAC_KEY environment variable is not set! Error: {}", e);
            std::process::exit(1);
        }
    };

    let redis_uri = match env::var("REDIS_URI") {
        Ok(uri) => uri,
        Err(e) => {
            println!("🔥 REDIS_URI environment variable is not set! Error: {}", e);
            std::process::exit(1);
        }
    };

    let application = Application::new(env, 8080, "info", &db_url, &redis_uri, &hmac_key).await?;
    application.run().await?;

    Ok(())
}
