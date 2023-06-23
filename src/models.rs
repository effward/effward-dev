use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct UserModel {
    pub id: u64,
    pub public_id: Vec<u8>,
    pub name: String,
    pub email_id: i64,
    pub password: String,
    pub is_deleted: i8,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct EmailModel {
    pub id: u64,
    pub address: String,
    pub created: NaiveDateTime,
}
