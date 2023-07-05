use async_trait::async_trait;
use chrono::{NaiveDateTime, Utc, TimeZone};
use sqlx::MySqlPool;
use uuid::Uuid;

use crate::entities::{EntityError, entity_stores::CachedSqlContentStore, utils, content::ContentStore};

use super::{Comment, comment_store::CommentStore};

const MIN_COMMENT_LENGTH: usize = 5;
const MAX_COMMENT_LENGTH: usize = 100_000;

#[derive(Clone)]
pub struct SqlCommentStore {
    pool: MySqlPool,
    content_store: CachedSqlContentStore
}

impl SqlCommentStore {
    pub fn new(pool: MySqlPool, content_store: CachedSqlContentStore) -> Self {
        Self { pool, content_store }
    }
}

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

impl From<CommentEntity> for Comment {
    fn from(comment_entity: CommentEntity) -> Self {
        Self {
            id: comment_entity.id,
            public_id: utils::get_readable_public_id(comment_entity.public_id),
            author_id: comment_entity.author_id,
            post_id: comment_entity.post_id,
            parent_id: comment_entity.parent_id,
            content_id: comment_entity.content_id,
            created: Utc.from_utc_datetime(&comment_entity.created),
            updated: Utc.from_utc_datetime(&comment_entity.updated),
        }
    }
}

#[async_trait]
impl CommentStore for SqlCommentStore {
    async fn insert(
        &self,
        author_id: &u64,
        post_id: &u64,
        parent_id: &Option<u64>,
        content: &str,
        ) -> Result<Comment, EntityError> {
        let comment_id = insert(&self.pool, &self.content_store, author_id, post_id, parent_id, content).await?;
        
        Ok(self.get_by_id(comment_id).await?)
    }

    async fn get_by_id(&self, id: u64) -> Result<Comment, EntityError> {
        Ok(Comment::from(get_by_id(&self.pool, id).await?))
    }

    async fn get_by_public_id(&self, public_id: &str) -> Result<Comment, EntityError> {
        let public_id = utils::parse_public_id(public_id)?;
        
        Ok(Comment::from(get_by_public_id(&self.pool, public_id).await?))
    }

    async fn get_count_by_post_id(&self, post_id: &u64) -> Result<i64, EntityError> {
        Ok(get_count_by_post_id(&self.pool, post_id).await?)
    }

    async fn get_by_post_id_parent_id(&self,
        post_id: u64,
        parent_id: Option<u64>,
        start_index: Option<u64>,
        count: u8,
        ) -> Result<Vec<Comment>, EntityError> {
        let comments = get_by_post_id_parent_id(&self.pool, post_id, parent_id, start_index, count).await?;
        
        let mut result: Vec<Comment> = vec![];
        
        for comment in comments {
            result.push(Comment::from(comment));
        }
        
        Ok(result)
    }
}

async fn insert(
    pool: &MySqlPool,
    content_store: &CachedSqlContentStore,
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
    if content.len() > MAX_COMMENT_LENGTH {
        return Err(EntityError::InvalidInput("content", "comment's content is too long"));
    }

    let content_id = content_store.get_or_create(content).await?;

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
        content_id.id,
        created,
        created
    )
    .execute(pool)
    .await?
    .last_insert_id();

    Ok(comment_id)
}

async fn get_by_id(pool: &MySqlPool, id: u64) -> Result<CommentEntity, EntityError> {
    Ok(sqlx::query_as!(
        CommentEntity,
    r#"
SELECT *
FROM comments
WHERE id = ?
    "#,
    id
    ).fetch_one(pool).await?)
}

async fn get_by_public_id(
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

async fn get_count_by_post_id(pool: &MySqlPool, post_id: &u64) -> Result<i64, EntityError> {
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

async fn get_by_post_id_parent_id(
    pool: &MySqlPool,
    post_id: u64,
    parent_id: Option<u64>,
    start_index: Option<u64>,
    count: u8,
    ) -> Result<Vec<CommentEntity>, EntityError> {
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

    Ok(comment_entities)
}
