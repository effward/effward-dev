use std::str;

use async_trait::async_trait;
use chrono::{NaiveDateTime, TimeZone, Utc};
use hex::ToHex;
use pbkdf2::pbkdf2_hmac_array;
use secrecy::{ExposeSecret, Secret};
use sha2::Sha256;
use sqlx::MySqlPool;
use uuid::Uuid;

use crate::entities::{email::EmailStore, entity_stores::CachedSqlEmailStore, utils, EntityError};

use super::{User, UserStore};

pub const MIN_USERNAME_LENGTH: usize = 4;
pub const MAX_USERNAME_LENGTH: usize = 32;
pub const MIN_PASSWORD_LENGTH: usize = 8;
pub const MAX_PASSWORD_LENGTH: usize = 256;

#[derive(Clone)]
pub struct SqlUserStore {
    pool: MySqlPool,
    email_store: CachedSqlEmailStore,
}

impl SqlUserStore {
    pub fn new(pool: MySqlPool, email_store: CachedSqlEmailStore) -> Self {
        Self { pool, email_store }
    }
}

#[derive(Debug, sqlx::FromRow)]
struct UserEntity {
    pub id: u64,
    pub public_id: Vec<u8>,
    pub name: String,
    pub email_id: u64,
    pub password: String,
    pub is_deleted: i8,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime,
}

impl From<UserEntity> for User {
    fn from(user_entity: UserEntity) -> Self {
        Self {
            id: user_entity.id,
            public_id: utils::get_readable_public_id(user_entity.public_id),
            name: user_entity.name,
            email_id: user_entity.email_id,
            is_deleted: user_entity.is_deleted > 0,
            created: Utc.from_utc_datetime(&user_entity.created),
            updated: Utc.from_utc_datetime(&user_entity.updated),
        }
    }
}

#[async_trait]
impl UserStore for SqlUserStore {
    async fn insert(
        &self,
        name: &str,
        email: &str,
        password: &Secret<String>,
    ) -> Result<User, EntityError> {
        let user_id = insert(&self.pool, &self.email_store, name, email, password).await?;

        Ok(self.get_by_id(user_id).await?)
    }

    async fn get_by_name_password(
        &self,
        name: &str,
        password: &Secret<String>,
    ) -> Result<User, EntityError> {
        Ok(User::from(
            get_by_name_password(&self.pool, name, password).await?,
        ))
    }

    async fn get_by_name(&self, name: &str) -> Result<User, EntityError> {
        Ok(User::from(get_by_name(&self.pool, name).await?))
    }

    async fn get_by_id(&self, id: u64) -> Result<User, EntityError> {
        Ok(User::from(get_by_id(&self.pool, id).await?))
    }

    async fn get_by_public_id(&self, public_id: &str) -> Result<User, EntityError> {
        let public_id = utils::parse_public_id(public_id)?;

        Ok(User::from(get_by_public_id(&self.pool, public_id).await?))
    }
}

async fn insert(
    pool: &MySqlPool,
    email_store: &CachedSqlEmailStore,
    name: &str,
    email: &str,
    password: &Secret<String>,
) -> Result<u64, EntityError> {
    if password.expose_secret().len() > MAX_PASSWORD_LENGTH {
        return Err(EntityError::InvalidInput(
            "password",
            "password is too long",
        ));
    }
    if password.expose_secret().len() < MIN_PASSWORD_LENGTH {
        return Err(EntityError::InvalidInput(
            "password",
            "password is too short",
        ));
    }

    let email = email_store.get_or_create(email).await?;

    let public_id = Uuid::new_v4().into_bytes();

    let salt_uuid = Uuid::new_v4().simple().to_string();
    let salt = salt_uuid[..6].as_bytes();
    let password = hash_password(password, salt);

    let created = Utc::now().naive_utc();

    let user_id = sqlx::query!(
        r#"
INSERT INTO users (public_id, name, email_id, password, is_deleted, created, updated)
VALUES (?, ?, ?, ?, ?, ?, ?)
        "#,
        &public_id[..],
        sanitize_name(name)?,
        email.id,
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

async fn get_by_name_password(
    pool: &MySqlPool,
    name: &str,
    password: &Secret<String>,
) -> Result<UserEntity, EntityError> {
    let user_entity = get_by_name(pool, name).await?;

    // password verification
    let parts: Vec<&str> = user_entity.password.split(':').collect();
    if parts.len() != 3 {
        return Err(EntityError::MalformedData);
    }
    let salt = parts[1];
    let password = hash_password(password, salt.as_bytes());

    if password == user_entity.password {
        Ok(user_entity)
    } else {
        Err(EntityError::InvalidInput("password", "incorrect password"))
    }
}

async fn get_by_name(pool: &MySqlPool, name: &str) -> Result<UserEntity, EntityError> {
    let user_entity = sqlx::query_as!(
        UserEntity,
        r#"
SELECT *
FROM users
WHERE name = ?
        "#,
        sanitize_name(name)?
    )
    .fetch_one(pool)
    .await?;

    Ok(user_entity)
}

async fn get_by_id(pool: &MySqlPool, id: u64) -> Result<UserEntity, EntityError> {
    let user_entity = sqlx::query_as!(
        UserEntity,
        r#"
SELECT *
FROM users
WHERE id = ?
        "#,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(user_entity)
}

async fn get_by_public_id(
    pool: &MySqlPool,
    public_id: Uuid,
) -> Result<UserEntity, EntityError> {
    let public_id_bytes = public_id.into_bytes();
    let user_entity = sqlx::query_as!(
        UserEntity,
        r#"
SELECT *
FROM users
WHERE public_id = ?
        "#,
        &public_id_bytes[..]
    )
    .fetch_one(pool)
    .await?;

    Ok(user_entity)
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

fn sanitize_name(name: &str) -> Result<String, EntityError> {
    Ok(
        utils::sanitize_text(name, MIN_USERNAME_LENGTH, MAX_USERNAME_LENGTH, "name")?
            .to_lowercase(),
    )
}
