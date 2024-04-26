use actix_session::storage::RedisSessionStore;
use log::warn;

use super::ServerError;

pub async fn init_session_store(redis_uri: &str) -> Result<RedisSessionStore, ServerError> {
    warn!("👤 Initializing session storage...");
    Ok(RedisSessionStore::new(redis_uri).await?)
}
