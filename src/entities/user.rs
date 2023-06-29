use std::str;

use chrono::{NaiveDateTime, Utc};
use hex::ToHex;
use pbkdf2::pbkdf2_hmac_array;
use secrecy::{ExposeSecret, Secret};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use sqlx::MySqlPool;
use uuid::Uuid;

use super::{email, EntityError, utils};

pub const MIN_USERNAME_LENGTH: usize = 4;
pub const MAX_USERNAME_LENGTH: usize = 32;
pub const MIN_PASSWORD_LENGTH: usize = 8;
pub const MAX_PASSWORD_LENGTH: usize = 256;

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct UserModel {
    pub id: u64,
    pub public_id: Vec<u8>,
    pub name: String,
    pub email_id: u64,
    pub password: String,
    pub is_deleted: i8,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime,
}

pub async fn create(
    pool: &MySqlPool,
    name: &String,
    email: &String,
    password: &Secret<String>,
) -> Result<u64, EntityError> {
    if password.expose_secret().len() > MAX_PASSWORD_LENGTH {
        return Err(EntityError::InvalidInput("password", "password is too long"));
    }
    if password.expose_secret().len() < MIN_PASSWORD_LENGTH {
        return Err(EntityError::InvalidInput("password", "password is too short"));
    }

    let email_id = email::get_or_create_id(pool, email).await?;

    let public_id = Uuid::new_v4().into_bytes();

    let salt_uuid = Uuid::new_v4().simple().to_string();
    let salt = salt_uuid[..6].as_bytes();
    let password = hash_password(password, salt);

    let created = Utc::now();

    let user_id = sqlx::query!(
        r#"
INSERT INTO users (public_id, name, email_id, password, is_deleted, created, updated)
VALUES (?, ?, ?, ?, ?, ?, ?)
        "#,
        &public_id[..],
        sanitize_name(name)?,
        email_id,
        password,
        0,
        created,
        created
    )
    .execute(pool)
    .await?
    .last_insert_id();

    Ok(user_id)
}

pub async fn get_by_name_password(
    pool: &MySqlPool,
    name: &String,
    password: &Secret<String>,
) -> Result<UserModel, EntityError> {
    let user = sqlx::query_as!(
        UserModel,
        r#"
SELECT *
FROM users
WHERE name = ?
        "#,
        sanitize_name(name)?
    )
    .fetch_one(pool)
    .await?;

    // password verification
    let parts: Vec<&str> = user.password.split(':').collect();
    if parts.len() != 3 {
        return Err(EntityError::MalformedData);
    }
    let salt = parts[1];
    let password = hash_password(password, salt.as_bytes());

    if password == user.password {
        Ok(user)
    } else {
        Err(EntityError::InvalidInput("password", "incorrect password"))
    }
}

pub async fn get_by_public_id(pool: &MySqlPool, public_id: Uuid) -> Result<UserModel, EntityError> {
    let public_id_bytes = public_id.into_bytes();
    let user = sqlx::query_as!(
        UserModel,
        r#"
SELECT *
FROM users
WHERE public_id = ?
        "#,
        &public_id_bytes[..]
    )
    .fetch_one(pool)
    .await?;

    Ok(user)
}

fn hash_password(password: &Secret<String>, salt: &[u8]) -> String {
    const HASH_FUNC: &str = "sha256_1024";
    const SEPARATOR: &str = ":";
    const N: u32 = 1024; // number of iterations

    let raw_password = password.expose_secret().as_bytes();
    let hash = pbkdf2_hmac_array::<Sha256, 20>(raw_password, salt, N);

    let hash_hex = hash.encode_hex::<String>();
    let salt_str = str::from_utf8(salt).unwrap();
    return hash_hex + SEPARATOR + salt_str + SEPARATOR + HASH_FUNC;
}

fn sanitize_name(name: &String) -> Result<String, EntityError> {
    utils::sanitize_text(name, MIN_USERNAME_LENGTH, MAX_USERNAME_LENGTH, "name")
}
