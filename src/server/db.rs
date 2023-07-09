use log::warn;
use sqlx::{mysql::MySqlPoolOptions, MySqlPool};

use super::ServerError;

pub async fn init_db(db_url: &str) -> Result<MySqlPool, ServerError> {
    let db_server = get_server(db_url)?;

    warn!("📚 Connecting to MySQL DB: {}", db_server);
    Ok(MySqlPoolOptions::new()
        .max_connections(10)
        .connect(db_url)
        .await?)
}

fn get_server(db_url: &str) -> Result<&str, ServerError> {
    let parts: Vec<&str> = db_url.split('@').collect();
    let num_parts = parts.len();
    if num_parts < 2 {
        return Err(ServerError::DatabaseInit(
            "📚🔥 DATABASE_URL string must contain at least one '@' character: {}".to_string(),
        ));
    }

    let last_part = parts[num_parts - 1];
    Ok(last_part)
}
