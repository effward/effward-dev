use async_trait::async_trait;
use chrono::Duration;

use crate::entities::{cache::Cache, EntityError};

use super::{Comment, CommentStore};

#[derive(Clone)]
pub struct CachedCommentStore<T>
where
    T: CommentStore,
{
    cache: Cache,
    source: T,
}

impl<T> CachedCommentStore<T>
where
    T: CommentStore,
{
    pub fn new(cache: Cache, source: T) -> Self {
        Self { cache, source }
    }
}

// TODO: set cache expiry, invalidate counts + collections, etc.

#[async_trait]
impl<T> CommentStore for CachedCommentStore<T>
where
    T: CommentStore + Send + Sync,
{
    async fn insert(
        &self,
        author_id: &u64,
        post_id: &u64,
        parent_id: &Option<u64>,
        content: &str,
        ) -> Result<Comment, EntityError> {
        self.cache.insert_cached(|| async {
            self.source.insert(author_id, post_id, parent_id, content).await },
        build_keys,
        None).await
    }

    async fn get_by_id(&self, id: u64) -> Result<Comment, EntityError> {
        let key = build_id_key(id);
        self.cache.get_cached(key, || async { self.source.get_by_id(id).await }, build_keys, None).await
    }

    async fn get_by_public_id(&self, public_id: &str) -> Result<Comment, EntityError> {
        let key = build_public_id_key(public_id);
        self.cache.get_cached(key, || async {
            self.source.get_by_public_id(public_id).await
        }, build_keys, None).await
    }

    async fn get_count_by_post_id(&self, post_id: &u64) -> Result<i64, EntityError> {
        let key = format!("count_by_post_id:{}", post_id);
        self.cache.get_cached(key.clone(), || async {
            self.source.get_count_by_post_id(post_id).await
        }, |_| { vec![key] }, Some(Duration::seconds(60))).await
    }

    async fn get_by_post_id_parent_id(&self,
                                      post_id: u64,
                                      parent_id: Option<u64>,
                                      start_index: Option<u64>,
                                      count: u8,
                                      ) -> Result<Vec<Comment>, EntityError> {
        let key = format!("post_id_parent_id:{}:{:?}:{:?}:{}", post_id, parent_id, start_index, count);
        self.cache.get_cached(key.clone(), || async {
            self.source.get_by_post_id_parent_id(post_id, parent_id, start_index, count).await
        }, |_| { vec![key] }, Some(Duration::seconds(60))).await
    }
}

fn build_keys(comment: &Comment) -> Vec<String> {
    vec![build_id_key(comment.id), build_public_id_key(&comment.public_id)]
}

fn build_id_key(id: u64) -> String {
    format!("id:{}", id)
}

fn build_public_id_key(public_id: &str) -> String {
    format!("public_id:{}", public_id)
}