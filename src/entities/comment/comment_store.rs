use async_trait::async_trait;

use crate::entities::EntityError;

use super::Comment;

#[async_trait]
pub trait CommentStore: Send + Sync + Clone {
    async fn insert(
        &self,
        author_id: &u64,
        post_id: &u64,
        parent_id: &Option<u64>,
        content: &str,
    ) -> Result<Comment, EntityError>;

    async fn get_by_id(&self, id: u64) -> Result<Comment, EntityError>;

    async fn get_by_public_id(&self, public_id: &str) -> Result<Comment, EntityError>;

    async fn get_count_by_post_id(&self, post_id: &u64) -> Result<i64, EntityError>;

    async fn get_by_post_id_parent_id(
        &self,
        post_id: u64,
        parent_id: Option<u64>,
        start_index: Option<u64>,
        count: u8,
    ) -> Result<Vec<Comment>, EntityError>;
}
