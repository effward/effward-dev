use std::sync::Arc;

use sqlx::MySqlPool;

use super::{
    cache::Cache,
    user::{CachedUserStore, SqlUserStore}, post::{SqlPostStore, CachedPostStore}, content::{CachedContentStore, SqlContentStore}, comment::{CachedCommentStore, SqlCommentStore},
};

pub type CachedSqlCommentStore = Arc<CachedCommentStore<SqlCommentStore>>;
pub type CachedSqlContentStore = Arc<CachedContentStore<SqlContentStore>>;
pub type CachedSqlPostStore = Arc<CachedPostStore<SqlPostStore>>;
pub type CachedSqlUserStore = Arc<CachedUserStore<SqlUserStore>>;


#[derive(Clone)]
pub struct EntityStores {
    pub comment_store: CachedSqlCommentStore,
    pub content_store: CachedSqlContentStore,
    pub post_store: CachedSqlPostStore,
    pub user_store: CachedSqlUserStore,
}

impl EntityStores {
    pub fn new(pool: MySqlPool) -> Self {
        let user_source = SqlUserStore::new(pool.clone());
        let user_store = Arc::new(CachedUserStore::new(Cache::new(), user_source));

        let content_source = SqlContentStore::new(pool.clone());
        let content_store = Arc::new(CachedContentStore::new(Cache::new(), content_source));

        let post_source = SqlPostStore::new(pool.clone(), content_store.clone());
        let post_store = Arc::new(CachedPostStore::new(Cache::new(), post_source));

        let comment_source = SqlCommentStore::new(pool.clone(), content_store.clone());
        let comment_store = Arc::new(CachedCommentStore::new(Cache::new(), comment_source));

        Self {
            comment_store,
            content_store,
            post_store,
            user_store,
        }
    }
}
