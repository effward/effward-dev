use std::future::ready;

use async_trait::async_trait;
use secrecy::Secret;

use crate::entities::{cache::Cache, EntityError};

use super::{User, UserStore};

#[derive(Clone)]
pub struct CachedUserStore<T>
where
    T: UserStore,
{
    cache: Cache,
    source: T,
}

impl<T> CachedUserStore<T>
where
    T: UserStore,
{
    pub fn new(cache: Cache, source: T) -> Self {
        Self { cache, source }
    }
}

async fn insert_source() -> Result<User, EntityError> {
    ready(Err(EntityError::DuplicateKey)).await
}

#[async_trait]
impl<T> UserStore for CachedUserStore<T>
where
    T: UserStore + Send + Sync,
{
    async fn insert(
        &self,
        name: &str,
        email: &str,
        password: &Secret<String>,
    ) -> Result<User, EntityError> {
        self.cache
            .insert_cached(
                insert_source,
                |user: User| {
                    vec![
                        format!("id:{}", user.id),
                        format!("public_id:{}", user.public_id),
                    ]
                },
                None,
            )
            .await
    }

    async fn get_by_name_password(
        &self,
        name: &str,
        password: &Secret<String>,
    ) -> Result<User, EntityError> {
        self.source.get_by_name_password(name, password).await
    }

    async fn get_by_name(&self, name: &str) -> Result<User, EntityError> {
        self.source.get_by_name(name).await
    }

    async fn get_by_id(&self, id: u64) -> Result<User, EntityError> {
        self.source.get_by_id(id).await
    }

    async fn get_by_public_id(&self, public_id: &str) -> Result<User, EntityError> {
        self.source.get_by_public_id(public_id).await
    }
}
