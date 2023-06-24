use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct EmailModel {
    pub id: u64,
    pub address: String,
    pub created: NaiveDateTime,
}

pub async fn get_or_create_id(pool: &MySqlPool, email: String) -> u64 {
    let created = Utc::now();
    let email_lower = email.to_lowercase();
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
            let new_id = sqlx::query!(
                r#"
INSERT INTO emails (address, created)
VALUES (?, ?)
                "#,
                email_lower,
                created
            )
            .execute(pool)
            .await
            .unwrap()
            .last_insert_id();

            new_id
        }
    };

    return email_id;
}
