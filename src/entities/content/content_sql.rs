use async_trait::async_trait;
use chrono::{NaiveDateTime, TimeZone, Utc};
use sqlx::MySqlPool;

use crate::entities::{utils, EntityError};

use super::{Content, ContentStore};

pub const MIN_CONTENT_LENGTH: usize = 1;
pub const MAX_CONTENT_LENGTH: usize = 16_777_215;

#[derive(Clone)]
pub struct SqlContentStore {
    pool: MySqlPool,
}

impl SqlContentStore {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

#[derive(Clone, Debug, sqlx::FromRow)]
pub struct ContentEntity {
    pub id: u64,
    pub body: String,
    pub body_hash: Vec<u8>,
    pub created: NaiveDateTime,
}

impl From<ContentEntity> for Content {
    fn from(content_entity: ContentEntity) -> Self {
        Self {
            id: content_entity.id,
            body: content_entity.body,
            body_hash: content_entity.body_hash,
            created: Utc.from_utc_datetime(&content_entity.created),
        }
    }
}

#[async_trait]
impl ContentStore for SqlContentStore {
    async fn insert(&self, body: &str) -> Result<Content, EntityError> {
        Ok(Content::from(insert(&self.pool, body).await?))
    }

    async fn get_or_create(&self, body: &str) -> Result<Content, EntityError> {
        Ok(Content::from(get_or_create(&self.pool, body).await?))
    }

    async fn get_by_id(&self, id: u64) -> Result<Content, EntityError> {
        Ok(Content::from(get_by_id(&self.pool, id).await?))
    }

    async fn get_by_body(&self, body: &str) -> Result<Content, EntityError> {
        Ok(Content::from(get_by_body(&self.pool, body).await?))
    }
}

async fn get_or_create(pool: &MySqlPool, body: &str) -> Result<ContentEntity, EntityError> {
    let body_hash = hash_body(body)?;
    match try_get_by_body_hash(pool, &body_hash).await {
        Ok(content_entity) => Ok(content_entity),
        Err(e) => match e {
            EntityError::NotFound => Ok(insert_by_body_hash(pool, body, &body_hash).await?),
            _ => Err(e),
        },
    }
}

async fn insert(pool: &MySqlPool, body: &str) -> Result<ContentEntity, EntityError> {
    let body_hash = hash_body(body)?;
    insert_by_body_hash(pool, body, &body_hash).await
}

async fn get_by_id(pool: &MySqlPool, id: u64) -> Result<ContentEntity, EntityError> {
    Ok(sqlx::query_as!(
        ContentEntity,
        r#"
SELECT *
FROM contents
WHERE id = ?
        "#,
        id
    )
    .fetch_one(pool)
    .await?)
}

async fn get_by_body(pool: &MySqlPool, body: &str) -> Result<ContentEntity, EntityError> {
    let body_hash = hash_body(body)?;
    try_get_by_body_hash(pool, &body_hash).await
}

async fn insert_by_body_hash(
    pool: &MySqlPool,
    body: &str,
    body_hash: &Vec<u8>,
) -> Result<ContentEntity, EntityError> {
    let created = Utc::now().naive_utc();
    let content_id = sqlx::query!(
        r#"
INSERT INTO contents (body, body_hash, created)
VALUES (?, ?, ?)
        "#,
        body,
        body_hash,
        created
    )
    .execute(pool)
    .await?
    .last_insert_id();

    Ok(ContentEntity {
        id: content_id,
        body: body.to_owned(),
        body_hash: body_hash.clone(),
        created,
    })
}

async fn try_get_by_body_hash(
    pool: &MySqlPool,
    body_hash: &Vec<u8>,
) -> Result<ContentEntity, EntityError> {
    Ok(sqlx::query_as!(
        ContentEntity,
        r#"
SELECT *
FROM contents
WHERE body_hash = ?
        "#,
        body_hash
    )
    .fetch_one(pool)
    .await?)
}

pub fn hash_body(body: &str) -> Result<Vec<u8>, EntityError> {
    utils::hash_content(body, MIN_CONTENT_LENGTH, MAX_CONTENT_LENGTH)
}
