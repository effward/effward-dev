use async_trait::async_trait;

use crate::entities::{cache::Cache, EntityError, utils};

use super::{Content, ContentStore, content_sql::hash_body};

#[derive(Clone)]
pub struct CachedContentStore<T>
where
    T: ContentStore,
{
    cache: Cache,
    source: T,
}

impl<T> CachedContentStore<T>
where
    T: ContentStore,
{
    pub fn new(cache: Cache, source: T) -> Self {
        Self { cache, source }
    }
}

#[async_trait]
impl<T> ContentStore for CachedContentStore<T>
where
    T: ContentStore + Send + Sync,
{
    async fn insert(
        &self,
        body: &str,
        ) -> Result<Content, EntityError> {
        self.cache.insert_cached(|| async {
            self.source.insert(body).await
        }, build_keys, None).await
    }

    async fn get_or_create(&self, body: &str) -> Result<Content, EntityError> {
        match self.get_by_body(body).await { // try get from cache
            Ok(content) => Ok(content),
            Err(_) => match self.insert(body).await { // insert into source
                Ok(content) => Ok(content),
                Err(entity_error) => match entity_error {
                    EntityError::DuplicateKey => { // if it already exists
                        let content = self.get_by_body(body).await?; // try to get it again
                        Ok(content)
                    },
                    _ => Err(entity_error)
                }
            }
        }
    }

    async fn get_by_id(&self, id: u64) -> Result<Content, EntityError> {
        let key = build_id_key(id);
        self.cache.get_cached(key, || async {
            self.source.get_by_id(id).await
        }, build_keys, None).await
    }
    
    async fn get_by_body(&self, body: &str) -> Result<Content, EntityError> {
        let body_hash = hash_body(body)?;
        let key = build_body_hash_key(body_hash);
        self.cache.get_cached(key, || async {
            self.source.get_by_body(body).await
        }, build_keys, None).await
    }
}

fn build_keys(content: &Content) -> Vec<String> {
    vec![build_id_key(content.id)]
}

fn build_id_key(id: u64) -> String {
    format!("id:{}", id)
}

fn build_body_hash_key(body_hash: Vec<u8>) -> String {
    format!("body_hash:{}", utils::vec_to_base64_string(body_hash))
}