use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use url::Url;
use uuid::Uuid;

use super::{EntityError, content, utils};

pub const MIN_TITLE_LENGTH: usize = 4;
pub const MAX_TITLE_LENGTH: usize = 400;

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct PostModel {
    pub id: u64,
    pub public_id: Vec<u8>,
    pub author: u64,
    pub title: String,
    pub link: Option<String>,
    pub content_id: Option<u64>,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime,
}

pub async fn create(
    pool: &MySqlPool,
    author: &u64,
    title: &String,
    link: &Option<String>,
    content: &Option<String>,
    ) -> Result<u64, EntityError> {
    // TODO: verify if author is a valid user?
    let sanitized_title = sanitize_title(title)?;
    verify_link(link)?;
    
    if content.is_none() && link.is_none() {
        return Err(EntityError::InvalidInput("post", "post must contain either a link or content (or both)"));
    }
    
    let content_id = match content {
        Some(c) => Some(content::get_or_create_id(pool, c).await?),
        None => None,
    };
    
    let public_id = Uuid::new_v4().into_bytes();
    let created = Utc::now();

    let post_id = sqlx::query!(
        r#"
INSERT INTO posts (public_id, author, title, link, content_id, created, updated)
VALUES (?, ?, ?, ?, ?, ?, ?)
        "#,
        &public_id[..],
        author,
        sanitized_title,
        link,
        content_id,
        created,
        created
    )
    .execute(pool)
    .await?
    .last_insert_id();

    Ok(post_id)
}

pub async fn get(pool: &MySqlPool, id: u64) -> Result<PostModel, EntityError> {
    let post = sqlx::query_as!(
        PostModel,
        r#"
SELECT *
FROM posts
WHERE id = ?
        "#,
        id
    )
    .fetch_one(pool)
    .await?;
    
    Ok(post)
}

pub async fn get_by_public_id(pool: &MySqlPool, public_id: Uuid) -> Result<PostModel, EntityError> {
    let public_id_bytes = public_id.into_bytes();
    let post = sqlx::query_as!(
        PostModel,
        r#"
SELECT *
FROM posts
WHERE public_id = ?
        "#,
        &public_id_bytes[..]
    )
    .fetch_one(pool)
    .await?;
    
    Ok(post)
}

pub async fn get_recent(pool: &MySqlPool, start_index: u64, count: u8) -> Result<Vec<PostModel>, EntityError> {
    let posts = sqlx::query_as!(
        PostModel,
        r#"
SELECT *
FROM posts
WHERE id < ?
ORDER BY id DESC
LIMIT ?
        "#,
        start_index,
        count
    )
    .fetch_all(pool)
    .await?;
    
    Ok(posts)
}

fn verify_link(link: &Option<String>) -> Result<(), EntityError> {
    match link {
        Some(l) => {
            match Url::parse(l) {
                Ok(_) => Ok(()),
                Err(_) => {
                    Err(EntityError::InvalidInput("link", "invalid url"))
                },
            }
        },
        None => Ok(()),
    }
}

fn sanitize_title(title: &String) -> Result<String, EntityError> {
    utils::sanitize_text(title, MIN_TITLE_LENGTH, MAX_TITLE_LENGTH, "title")
}