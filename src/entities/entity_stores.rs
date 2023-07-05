use std::sync::Arc;

use sqlx::MySqlPool;

use super::{
    cache::Cache,
    user::{CachedUserStore, SqlUserStore}, post::{SqlPostStore, CachedPostStore},
};

#[derive(Clone)]
pub struct EntityStores {
    pub user_store: Arc<CachedUserStore<SqlUserStore>>,
    pub post_store: Arc<CachedPostStore<SqlPostStore>>,
}

impl EntityStores {
    pub fn new(pool: MySqlPool) -> Self {
        let user_source = SqlUserStore::new(pool.clone());
        let user_store = CachedUserStore::new(Cache::new(), user_source);

        let post_source = SqlPostStore::new(pool.clone());
        let post_store = CachedPostStore::new(Cache::new(), post_source);
        Self { user_store: Arc::new(user_store), post_store: Arc::new(post_store) }
    }
}
