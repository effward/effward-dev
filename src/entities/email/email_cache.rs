use async_trait::async_trait;

use crate::entities::{cache::Cache, EntityError};

use super::{Email, EmailStore};

#[derive(Clone)]
pub struct CachedEmailStore<T>
where
    T: EmailStore,
{
    cache: Cache,
    source: T,
}

impl<T> CachedEmailStore<T>
where
    T: EmailStore,
{
    pub fn new(cache: Cache, source: T) -> Self {
        Self { cache, source }
    }
}

#[async_trait]
impl<T> EmailStore for CachedEmailStore<T>
where
    T: EmailStore + Send + Sync,
{
    async fn get_or_create(&self, address: &str) -> Result<Email, EntityError> {
        let key = build_address_key(address);
        self.cache
            .get_cached(
                key,
                || async { self.source.get_or_create(address).await },
                build_keys,
                None,
            )
            .await
    }

    async fn get_by_id(&self, id: u64) -> Result<Email, EntityError> {
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

    async fn get_by_address(&self, address: &str) -> Result<Email, EntityError> {
        let key = build_address_key(address);

        self.cache
            .get_cached(
                key,
                || async { self.source.get_by_address(address).await },
                build_keys,
                None,
            )
            .await
    }
}

fn build_keys(email: &Email) -> Vec<String> {
    vec![build_id_key(email.id), build_address_key(&email.address)]
}

fn build_id_key(id: u64) -> String {
    format!("id:{}", id)
}

fn build_address_key(address: &str) -> String {
    format!("address:{}", address)
}
