use chrono::{NaiveDateTime, Utc};
use email_address::*;
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;

use super::EntityError;

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct EmailModel {
    pub id: u64,
    pub address: String,
    pub created: NaiveDateTime,
}

pub async fn get_or_create_id(pool: &MySqlPool, email: &String) -> Result<u64, EntityError> {
    let email_lower = email.to_lowercase();
    if !EmailAddress::is_valid(&email_lower) {
        return Err(EntityError::InvalidInput("email", "email is invalid"));
    }

    let email_entity = try_get_by_address(pool, &email_lower).await?;
    match email_entity {
        Some(e) => Ok(e.id),
        None => Ok(insert(pool, &email_lower).await?)
    }
}

async fn insert(
    pool: &MySqlPool,
    address: &String,
) -> Result<u64, EntityError> {
    let created = Utc::now();
    let email_id = sqlx::query!(
        r#"
INSERT INTO emails (address, created)
VALUES (?, ?)
        "#,
        address,
        created
    )
    .execute(pool)
    .await?
    .last_insert_id();

    Ok(email_id)
}

async fn try_get_by_address(
    pool: &MySqlPool,
    address: &String,
    ) -> Result<Option<EmailModel>, EntityError> {
    let email_entity = sqlx::query_as!(
        EmailModel,
        r#"
SELECT *
FROM emails
WHERE address = ?
        "#,
        address
    )
    .fetch_optional(pool)
    .await?;

    Ok(email_entity)
}