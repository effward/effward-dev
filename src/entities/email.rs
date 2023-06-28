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
        return Err(EntityError::InvalidInput("email".to_owned()));
    }

    let created = Utc::now();

    let email_id = match sqlx::query_as!(
        EmailModel,
        r#"
SELECT *
FROM emails
WHERE address = ?
        "#,
        email_lower
    )
    .fetch_one(pool)
    .await
    {
        Ok(email) => email.id,
        Err(_err) => {
            let create_result = sqlx::query!(
                r#"
INSERT INTO emails (address, created)
VALUES (?, ?)
                "#,
                email_lower,
                created
            )
            .execute(pool)
            .await?;

            create_result.last_insert_id()
        }
    };

    return Ok(email_id);
}
