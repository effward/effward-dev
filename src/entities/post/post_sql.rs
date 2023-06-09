use async_trait::async_trait;
use chrono::{NaiveDateTime, TimeZone, Utc};
use log::error;
use sqlx::MySqlPool;
use url::Url;
use uuid::Uuid;

use crate::entities::{
    content::ContentStore, entity_stores::CachedSqlContentStore, utils, EntityError,
};

use super::{Post, PostStore};

pub const MIN_TITLE_LENGTH: usize = 4;
pub const MAX_TITLE_LENGTH: usize = 400;

#[derive(Clone)]
pub struct SqlPostStore {
    pool: MySqlPool,
    content_store: CachedSqlContentStore,
}

impl SqlPostStore {
    pub fn new(pool: MySqlPool, content_store: CachedSqlContentStore) -> Self {
        Self {
            pool,
            content_store,
        }
    }
}

#[derive(Debug, sqlx::FromRow)]
pub struct PostEntity {
    pub id: u64,
    pub public_id: Vec<u8>,
    pub author_id: u64,
    pub title: String,
    pub link: Option<String>,
    pub content_id: Option<u64>,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime,
}

impl From<PostEntity> for Post {
    fn from(post_entity: PostEntity) -> Self {
        Self {
            id: post_entity.id,
            public_id: utils::get_readable_public_id(post_entity.public_id),
            author_id: post_entity.author_id,
            title: post_entity.title,
            link: post_entity.link,
            content_id: post_entity.content_id,
            created: Utc.from_utc_datetime(&post_entity.created),
            updated: Utc.from_utc_datetime(&post_entity.updated),
        }
    }
}

#[async_trait]
impl PostStore for SqlPostStore {
    async fn insert(
        &self,
        author_id: &u64,
        title: &str,
        link: &Option<String>,
        content: &Option<String>,
    ) -> Result<Post, EntityError> {
        let post_id = insert(
            &self.pool,
            &self.content_store,
            author_id,
            title,
            link,
            content,
        )
        .await?;

        Ok(self.get_by_id(post_id).await?)
    }

    async fn get_by_id(&self, id: u64) -> Result<Post, EntityError> {
        Ok(Post::from(get_by_id(&self.pool, id).await?))
    }

    async fn get_by_public_id(&self, public_id: &str) -> Result<Post, EntityError> {
        let public_id = utils::parse_public_id(public_id)?;

        Ok(Post::from(get_by_public_id(&self.pool, public_id).await?))
    }

    async fn get_recent(
        &self,
        start_index: Option<u64>,
        count: u8,
    ) -> Result<Vec<Post>, EntityError> {
        let recent_posts = get_recent(&self.pool, start_index, count).await?;
        let mut posts: Vec<Post> = vec![];

        for post in recent_posts {
            posts.push(Post::from(post));
        }

        Ok(posts)
    }
}

async fn insert(
    pool: &MySqlPool,
    content_store: &CachedSqlContentStore,
    author_id: &u64,
    title: &str,
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
        Some(c) => {
            let content = content_store.get_or_create(c).await?;
            Some(content.id)
        }
        None => None,
    };

    let public_id = Uuid::new_v4().into_bytes();
    let created = Utc::now().naive_utc();

    let post_id = sqlx::query!(
        r#"
INSERT INTO posts (public_id, author_id, title, link, content_id, created, updated)
VALUES (?, ?, ?, ?, ?, ?, ?)
        "#,
        &public_id[..],
        author_id,
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

async fn get_by_id(pool: &MySqlPool, id: u64) -> Result<PostEntity, EntityError> {
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

async fn get_by_public_id(pool: &MySqlPool, public_id: Uuid) -> Result<PostEntity, EntityError> {
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

async fn get_recent(
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
        Some(l) => match Url::parse(l) {
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

fn sanitize_title(title: &str) -> Result<String, EntityError> {
    utils::sanitize_text(title, MIN_TITLE_LENGTH, MAX_TITLE_LENGTH, "title")
}
