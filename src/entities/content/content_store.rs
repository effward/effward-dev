use async_trait::async_trait;

use crate::entities::EntityError;

use super::Content;

#[async_trait]
pub trait ContentStore: Send + Sync + Clone {
    async fn insert(
        &self,
        body: &str,
        ) -> Result<Content, EntityError>;

    async fn get_or_create(&self, body: &str) -> Result<Content, EntityError>;
    
    async fn get_by_id(&self, id: u64) -> Result<Content, EntityError>;
    
    async fn get_by_body(&self, body: &str) -> Result<Content, EntityError>;
}
