use sqlx::MySqlPool;

use super::{
    cache::Cache,
    user::{CachedUserStore, SqlUserStore},
};

#[derive(Clone)]
pub struct EntityStores {
    pub user_store: CachedUserStore<SqlUserStore>,
}

impl EntityStores {
    pub fn new(pool: MySqlPool) -> Self {
        let cache = Cache::new();

        let user_source = SqlUserStore::new(pool);
        let user_store = CachedUserStore::new(cache, user_source);

        Self { user_store }
    }
}
