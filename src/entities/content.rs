use cached::proc_macro::cached;
use chrono::{NaiveDateTime, Utc};
use log::error;
use sha2::{Digest, Sha256};
use sqlx::MySqlPool;

use super::EntityError;

pub const MIN_CONTENT_LENGTH: usize = 1;
pub const MAX_CONTENT_LENGTH: usize = 16_777_215;

#[derive(Clone, Debug, sqlx::FromRow)]
pub struct ContentEntity {
    pub id: u64,
    pub body: String,
    pub body_hash: Vec<u8>,
    pub created: NaiveDateTime,
}

pub async fn get_or_create_id(pool: &MySqlPool, content: &str) -> Result<u64, EntityError> {
    let body_hash = hash_content(content)?;
    let content_entity = try_get_by_body_hash(pool, content, &body_hash).await?;

    let content_id = match content_entity {
        Some(c) => c.id,
        None => insert_by_body_hash(pool, content, &body_hash).await?,
    };

    Ok(content_id)
}

pub async fn insert(pool: &MySqlPool, content: &str) -> Result<u64, EntityError> {
    let body_hash = hash_content(content)?;
    insert_by_body_hash(pool, content, &body_hash).await
}

#[cached(name = "CONTENT_BY_ID", key = "String", convert = r#"{ format!("{}", id) }"#, size = 100, time = 300, result = true)]
pub async fn get_by_id(pool: &MySqlPool, id: u64) -> Result<ContentEntity, EntityError> {
    let content_entity = sqlx::query_as!(
        ContentEntity,
        r#"
SELECT *
FROM contents
WHERE id = ?
        "#,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(content_entity)
}

pub async fn get_by_content(
    pool: &MySqlPool,
    content: &str,
) -> Result<Option<ContentEntity>, EntityError> {
    let body_hash = hash_content(content)?;
    try_get_by_body_hash(pool, content, &body_hash).await
}

async fn insert_by_body_hash(
    pool: &MySqlPool,
    content: &str,
    body_hash: &Vec<u8>,
) -> Result<u64, EntityError> {
    let created = Utc::now().naive_utc();
    let content_id = sqlx::query!(
        r#"
INSERT INTO contents (body, body_hash, created)
VALUES (?, ?, ?)
        "#,
        content,
        body_hash,
        created
    )
    .execute(pool)
    .await?
    .last_insert_id();

    Ok(content_id)
}

async fn try_get_by_body_hash(
    pool: &MySqlPool,
    content: &str,
    body_hash: &Vec<u8>,
) -> Result<Option<ContentEntity>, EntityError> {
    let content_entity = sqlx::query_as!(
        ContentEntity,
        r#"
SELECT *
FROM contents
WHERE body_hash = ?
        "#,
        body_hash
    )
    .fetch_optional(pool)
    .await?;

    if let Some(ce) = content_entity.clone() {
        if ce.body == content.to_owned() {
            error!(
                "🔥🔥🔥 SHA256 collision for two different contents: {}\n\nand\n\n{}",
                ce.body, content
            );
            return Err(EntityError::MalformedData);
        }
    }

    Ok(content_entity)
}

fn hash_content(content: &str) -> Result<Vec<u8>, EntityError> {
    if content.len() < MIN_CONTENT_LENGTH {
        return Err(EntityError::InvalidInput("content", "content is too short"));
    }
    if content.len() > MAX_CONTENT_LENGTH {
        return Err(EntityError::InvalidInput("content", "content is too long"));
    }

    let mut hasher = Sha256::new();
    hasher.update(content);

    let hash = hasher.finalize()[..].to_vec();
    Ok(hash)
}
