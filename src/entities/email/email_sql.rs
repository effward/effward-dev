use async_trait::async_trait;
use chrono::{NaiveDateTime, TimeZone, Utc};
use email_address::*;
use sqlx::MySqlPool;

use crate::entities::EntityError;

use super::{Email, EmailStore};

#[derive(Clone)]
pub struct SqlEmailStore {
    pool: MySqlPool,
}

impl SqlEmailStore {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

#[derive(Debug, sqlx::FromRow)]
pub struct EmailEntity {
    pub id: u64,
    pub address: String,
    pub created: NaiveDateTime,
}

impl From<EmailEntity> for Email {
    fn from(email_entity: EmailEntity) -> Self {
        Self {
            id: email_entity.id,
            address: email_entity.address,
            created: Utc.from_utc_datetime(&email_entity.created),
        }
    }
}

#[async_trait]
impl EmailStore for SqlEmailStore {
    async fn get_or_create(&self, address: &str) -> Result<Email, EntityError> {
        let email_id = get_or_create_id(&self.pool, address).await?;

        self.get_by_id(email_id).await
    }

    async fn get_by_id(&self, id: u64) -> Result<Email, EntityError> {
        Ok(Email::from(get_by_id(&self.pool, id).await?))
    }

    async fn get_by_address(&self, address: &str) -> Result<Email, EntityError> {
        match try_get_by_address(&self.pool, address).await? {
            Some(email_entity) => Ok(Email::from(email_entity)),
            None => Err(EntityError::NotFound),
        }
    }
}

async fn get_or_create_id(pool: &MySqlPool, address: &str) -> Result<u64, EntityError> {
    let email_lower = address.to_lowercase();
    if !EmailAddress::is_valid(&email_lower) {
        return Err(EntityError::InvalidInput("email", "email is invalid"));
    }

    let email_entity = try_get_by_address(pool, &email_lower).await?;
    match email_entity {
        Some(e) => Ok(e.id),
        None => Ok(insert(pool, &email_lower).await?),
    }
}

async fn insert(pool: &MySqlPool, address: &str) -> Result<u64, EntityError> {
    Ok(sqlx::query!(
        r#"
INSERT INTO emails (address, created)
VALUES (?, ?)
        "#,
        address,
        Utc::now().naive_utc()
    )
    .execute(pool)
    .await?
    .last_insert_id())
}

async fn get_by_id(pool: &MySqlPool, id: u64) -> Result<EmailEntity, EntityError> {
    Ok(sqlx::query_as!(
        EmailEntity,
        r#"
SELECT *
FROM emails
WHERE id = ?
    "#,
        id
    )
    .fetch_one(pool)
    .await?)
}

async fn try_get_by_address(
    pool: &MySqlPool,
    address: &str,
) -> Result<Option<EmailEntity>, EntityError> {
    Ok(sqlx::query_as!(
        EmailEntity,
        r#"
SELECT *
FROM emails
WHERE address = ?
        "#,
        address
    )
    .fetch_optional(pool)
    .await?)
}
