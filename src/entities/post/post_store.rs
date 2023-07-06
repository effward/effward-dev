use async_trait::async_trait;

use crate::entities::EntityError;

use super::Post;

#[async_trait]
pub trait PostStore: Send + Sync + Clone {
    async fn insert(
        &self,
        author_id: &u64,
        title: &str,
        link: &Option<String>,
        content: &Option<String>,
    ) -> Result<Post, EntityError>;

    async fn get_by_id(&self, id: u64) -> Result<Post, EntityError>;

    async fn get_by_public_id(&self, public_id: &str) -> Result<Post, EntityError>;

    async fn get_recent(
        &self,
        start_index: Option<u64>,
        count: u8,
    ) -> Result<Vec<Post>, EntityError>;
}
