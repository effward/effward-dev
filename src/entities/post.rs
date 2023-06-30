use chrono::{NaiveDateTime, Utc};
use log::error;
use sqlx::MySqlPool;
use url::Url;
use uuid::Uuid;

use super::{content, utils, EntityError};

pub const MIN_TITLE_LENGTH: usize = 4;
pub const MAX_TITLE_LENGTH: usize = 400;

#[derive(Debug, sqlx::FromRow)]
pub struct PostEntity {
    pub id: u64,
    pub public_id: Vec<u8>,
    pub author: u64,
    pub title: String,
    pub link: Option<String>,
    pub content_id: Option<u64>,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime,
}

pub async fn insert(
    pool: &MySqlPool,
    author: &u64,
    title: &String,
    link: &Option<String>,
    content: &Option<String>,
) -> Result<u64, EntityError> {
    // TODO: verify if author is a valid user?
    let sanitized_title = sanitize_title(title)?;
    let link = verify_link(link)?;

    if content.is_none() && link.is_none() {
        return Err(EntityError::InvalidInput(
            "post",
            "post must contain either a link or content (or both)",
        ));
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

pub async fn get_by_id(pool: &MySqlPool, id: u64) -> Result<PostEntity, EntityError> {
    let post_entity = sqlx::query_as!(
        PostEntity,
        r#"
SELECT *
FROM posts
WHERE id = ?
        "#,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(post_entity)
}

pub async fn get_by_public_id(
    pool: &MySqlPool,
    public_id: Uuid,
) -> Result<PostEntity, EntityError> {
    let public_id_bytes = public_id.into_bytes();
    let post_entity = sqlx::query_as!(
        PostEntity,
        r#"
SELECT *
FROM posts
WHERE public_id = ?
        "#,
        &public_id_bytes[..]
    )
    .fetch_one(pool)
    .await?;

    Ok(post_entity)
}

pub async fn get_recent(
    pool: &MySqlPool,
    start_index: Option<u64>,
    count: u8,
) -> Result<Vec<PostEntity>, EntityError> {
    let post_entities = match start_index {
        Some(start_index) => {
            sqlx::query_as!(
                PostEntity,
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
            .await?
        }
        None => {
            sqlx::query_as!(
                PostEntity,
                r#"
    SELECT *
    FROM posts
    ORDER BY id DESC
    LIMIT ?
            "#,
                count
            )
            .fetch_all(pool)
            .await?
        }
    };

    Ok(post_entities)
}

fn verify_link(link: &Option<String>) -> Result<Option<String>, EntityError> {
    match link {
        Some(l) => match Url::parse(&l) {
            Ok(_) => Ok(link.to_owned()),
            Err(e) => {
                if l.is_empty() {
                    return Ok(None);
                }

                error!("l: {}, URL Parse error: {:?}", l, e);
                Err(EntityError::InvalidInput("link", "invalid url"))
            }
        },
        None => Ok(None),
    }
}

fn sanitize_title(title: &String) -> Result<String, EntityError> {
    utils::sanitize_text(title, MIN_TITLE_LENGTH, MAX_TITLE_LENGTH, "title")
}
