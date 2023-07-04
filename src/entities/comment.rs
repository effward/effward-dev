use cached::{Cached, proc_macro::cached, TimedSizedCache};
use chrono::{NaiveDateTime, Utc};
use sqlx::MySqlPool;
use uuid::Uuid;

use super::{content, EntityError};

#[derive(Clone, Debug, sqlx::FromRow)]
pub struct CommentEntity {
    pub id: u64,
    pub public_id: Vec<u8>,
    pub author_id: u64,
    pub post_id: u64,
    pub parent_id: Option<u64>,
    pub content_id: u64,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime,
}

const MIN_COMMENT_LENGTH: usize = 5;

pub async fn insert(
    pool: &MySqlPool,
    author_id: &u64,
    post_id: &u64,
    parent_id: &Option<u64>,
    content: &str,
) -> Result<u64, EntityError> {
    // TODO: verify if author is a valid user?

    if content.len() < MIN_COMMENT_LENGTH {
        return Err(EntityError::InvalidInput(
            "content",
            "comment's content is too short",
        ));
    }

    let content_id = content::get_or_create_id(pool, content).await?;

    let public_id = Uuid::new_v4().into_bytes();
    let created = Utc::now().naive_utc();

    let comment_id = sqlx::query!(
        r#"
INSERT INTO comments
    (public_id, author_id, post_id, parent_id, content_id, created, updated)
VALUES
    (?, ?, ?, ?, ?, ?, ?)
        "#,
        &public_id[..],
        author_id,
        post_id,
        parent_id,
        content_id,
        created,
        created
    )
    .execute(pool)
    .await?
    .last_insert_id();

    Ok(comment_id)
}

pub async fn get_by_public_id(
    pool: &MySqlPool,
    public_id: Uuid,
) -> Result<CommentEntity, EntityError> {
    let public_id_bytes = public_id.into_bytes();
    let comment_entity = sqlx::query_as!(
        CommentEntity,
        r#"
SELECT *
FROM comments
WHERE public_id = ?
        "#,
        &public_id_bytes[..]
    )
    .fetch_one(pool)
    .await?;

    Ok(comment_entity)
}

pub async fn get_count_by_post_id(pool: &MySqlPool, post_id: &u64) -> Result<i64, EntityError> {
    let count = sqlx::query!(
        r#"
SELECT
    COUNT(id) as count
FROM comments
WHERE post_id = ?
        "#,
        post_id
    )
    .fetch_one(pool)
    .await?;

    Ok(count.count)
}

// #[cached(name = "COMMENT_POST_ID_PARENT_ID", key = "String", convert = r#"{ format!("{}::{:?}::{:?}::{}", post_id, parent_id, start_index, count) }"#, size = 100, time = 300, result = true)]
pub async fn get_by_post_id_parent_id(
    pool: &MySqlPool,
    cache: &mut TimedSizedCache<String, Vec<CommentEntity>>,
    post_id: &u64,
    parent_id: Option<u64>,
    start_index: Option<u64>,
    count: u8,
) -> Result<Vec<CommentEntity>, EntityError> {
    let key = format!("{}::{:?}::{:?}::{}", post_id, parent_id, start_index, count);

    if let Some(cached_value) = cache.cache_get(&key) {
        return Ok(cached_value.clone());
    }

    let comment_entities = match parent_id {
        Some(parent_id) => match start_index {
            Some(start_index) => {
                sqlx::query_as!(
                    CommentEntity,
                    r#"
SELECT *
FROM `comments`
WHERE
    `post_id` = ?
    AND `parent_id` = ?
    AND `id` > ?
ORDER BY
    `id` ASC
LIMIT ?
                "#,
                    post_id,
                    parent_id,
                    start_index,
                    count
                )
                .fetch_all(pool)
                .await?
            }
            None => {
                sqlx::query_as!(
                    CommentEntity,
                    r#"
SELECT *
FROM `comments`
WHERE
    `post_id` = ?
    AND `parent_id` = ?
ORDER BY
    `id` ASC
LIMIT ?
                "#,
                    post_id,
                    parent_id,
                    count
                )
                .fetch_all(pool)
                .await?
            }
        },
        None => match start_index {
            Some(start_index) => {
                sqlx::query_as!(
                    CommentEntity,
                    r#"
SELECT *
FROM `comments`
WHERE
    `post_id` = ?
    AND `parent_id` IS NULL
    AND `id` > ?
ORDER BY
    `id` ASC
LIMIT ?
                "#,
                    post_id,
                    start_index,
                    count
                )
                .fetch_all(pool)
                .await?
            }
            None => {
                sqlx::query_as!(
                    CommentEntity,
                    r#"
SELECT *
FROM `comments`
WHERE
    `post_id` = ?
    AND `parent_id` IS NULL
ORDER BY
    `id` ASC
LIMIT ?
                "#,
                    post_id,
                    count
                )
                .fetch_all(pool)
                .await?
            }
        },
    };
    cache.cache_set(key, comment_entities.clone());
    Ok(comment_entities)
}
