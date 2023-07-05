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
                || async { self.source.insert(name, email, password).await },
                build_keys,
                None,
            )
            .await
    }

    async fn get_by_name_password(
        &self,
        name: &str,
        password: &Secret<String>,
    ) -> Result<User, EntityError> {
        // Never cache because it's used for password checks
        self.source.get_by_name_password(name, password).await
    }

    async fn get_by_name(&self, name: &str) -> Result<User, EntityError> {
        let key = build_name_key(name);
        self.cache
            .get_cached(
                key,
                || async { self.source.get_by_name(name).await },
                build_keys,
                None,
            )
            .await
    }

    async fn get_by_id(&self, id: u64) -> Result<User, EntityError> {
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

    async fn get_by_public_id(&self, public_id: &str) -> Result<User, EntityError> {
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
}

fn build_keys(user: &User) -> Vec<String> {
    vec![
        build_id_key(user.id),
        build_public_id_key(&user.public_id),
        build_name_key(&user.name),
    ]
}

fn build_id_key(id: u64) -> String {
    format!("id:{}", id)
}

fn build_public_id_key(public_id: &str) -> String {
    format!("public_id:{}", public_id)
}

fn build_name_key(name: &str) -> String {
    format!("name:{}", name)
}
