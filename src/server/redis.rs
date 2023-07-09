use log::warn;
use redis::aio::ConnectionManager;

use super::ServerError;

pub async fn init_redis(redis_uri: &str) -> Result<ConnectionManager, ServerError> {
    warn!("🗝️ Connecting to redis...");
    let client = redis::Client::open(redis_uri)?;
    let conn_manager = ConnectionManager::new(client).await?;

    Ok(conn_manager)
}
