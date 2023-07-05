use async_trait::async_trait;
use chrono::Duration;

use crate::entities::{cache::Cache, EntityError};

use super::{Post, PostStore};

#[derive(Clone)]
pub struct CachedPostStore<T>
where
    T: PostStore,
{
    cache: Cache,
    source: T,
}

impl<T> CachedPostStore<T>
where
    T: PostStore,
{
    pub fn new(cache: Cache, source: T) -> Self {
        Self { cache, source }
    }
}

// TODO: set cache expiry, invalidate collections, etc.

#[async_trait]
impl<T> PostStore for CachedPostStore<T>
where
    T: PostStore + Send + Sync,
{
    async fn insert(
        &self,
        author_id: &u64,
        title: &String,
        link: &Option<String>,
        content: &Option<String>,
    ) -> Result<Post, EntityError> {
        self.cache
            .insert_cached(
                || async { self.source.insert(author_id, title, link, content).await },
                build_keys,
                None,
            )
            .await
    }

    async fn get_by_id(&self, id: u64) -> Result<Post, EntityError> {
        let key = build_id_key(id);
        self.cache
            .get_cached(
                key,
                || async { self.source.get_by_id(id).await },
                build_keys,
                None,
            )
            .await
    }

    async fn get_by_public_id(&self, public_id: &str) -> Result<Post, EntityError> {
        let key = build_public_id_key(public_id);
        self.cache
            .get_cached(
                key,
                || async { self.source.get_by_public_id(public_id).await },
                build_keys,
                None,
            )
            .await
    }

    async fn get_recent(
        &self,
        start_index: Option<u64>,
        count: u8,
    ) -> Result<Vec<Post>, EntityError> {
        let key = format!("recent:{:?}:{}", start_index, count);
        let expiry = match start_index {
            Some(_) => Duration::minutes(60),
            None => Duration::seconds(60),
        };
        self.cache
            .get_cached(
                key.clone(),
                || async { self.source.get_recent(start_index, count).await },
                |_| vec![key],
                Some(expiry),
            )
            .await
    }
}

fn build_keys(post: &Post) -> Vec<String> {
    vec![build_id_key(post.id), build_public_id_key(&post.public_id)]
}

fn build_id_key(id: u64) -> String {
    format!("id:{}", id)
}

fn build_public_id_key(public_id: &str) -> String {
    format!("public_id:{}", public_id)
}
