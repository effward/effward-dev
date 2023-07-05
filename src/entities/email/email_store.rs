use async_trait::async_trait;

use crate::entities::EntityError;

use super::Email;

#[async_trait]
pub trait EmailStore: Send + Sync + Clone {
    async fn get_or_create(&self, address: &str) -> Result<Email, EntityError>;

    async fn get_by_id(&self, id: u64) -> Result<Email, EntityError>;

    async fn get_by_address(&self, address: &str) -> Result<Email, EntityError>;
}
