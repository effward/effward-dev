use async_trait::async_trait;
use secrecy::Secret;

use crate::entities::{cache::Cache, EntityError};

use super::{User, UserStore};

#[derive(Clone)]
pub struct CachedUserStore<T>
where
    T: UserStore,
{
    source: T,
}

impl<T> CachedUserStore<T>
where
    T: UserStore,
{
    pub fn new(cache: Cache, source: T) -> Self {
        Self { source }
    }
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
    ) -> Result<u64, EntityError> {
        self.source.insert(name, email, password).await
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
