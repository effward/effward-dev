use actix_web_flash_messages::{IncomingFlashMessages, Level};
use log::error;
use serde::{Deserialize, Serialize};
use shortguid::ShortGuid;
use sqlx::MySqlPool;
use tera::Context;

use crate::{
    entities::user::{self, UserEntity},
    routes::models::{self, User},
};

use super::{TypedSession, UserContextError};

pub struct UserContext {
    pub auth_user: Option<User>,
    pub context: Context,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Notifications {
    pub errors: Vec<String>,
    pub warns: Vec<String>,
    pub infos: Vec<String>,
    pub successes: Vec<String>,
}

pub async fn build_user_context(
    session: TypedSession,
    flash_messages: IncomingFlashMessages,
    pool: &MySqlPool,
    page_name: &str,
) -> UserContext {
    let mut context = Context::new();

    insert_title(&mut context, page_name);
    insert_notifications(&mut context, flash_messages);
    let auth_user = insert_auth_user(&mut context, session, pool).await;

    UserContext { auth_user, context }
}

pub async fn get_auth_user_entity(
    session: TypedSession,
    pool: &MySqlPool,
) -> Result<UserEntity, UserContextError> {
    match session.get_user_id()? {
        None => Err(UserContextError::NotAuthenticated),
        Some(user_id) => {
            let short_uuid = ShortGuid::try_parse(user_id)?;
            let auth_user_entity = user::get_by_public_id(&pool, *short_uuid.as_uuid()).await?;

            Ok(auth_user_entity)
        }
    }
}

fn insert_title(context: &mut Context, page_name: &str) {
    context.insert("title", &format!("effward.dev - {}", page_name));
}

fn insert_notifications(context: &mut Context, flash_messages: IncomingFlashMessages) {
    let mut errors: Vec<String> = vec![];
    let mut warns: Vec<String> = vec![];
    let mut infos: Vec<String> = vec![];
    let mut successes: Vec<String> = vec![];
    for m in flash_messages.iter() {
        let content = String::from(m.content());
        match m.level() {
            Level::Debug => (),
            Level::Info => infos.push(content),
            Level::Success => successes.push(content),
            Level::Warning => warns.push(content),
            Level::Error => errors.push(content),
        }
    }
    context.insert(
        "notifications",
        &Notifications {
            errors,
            warns,
            infos,
            successes,
        },
    );
}

async fn insert_auth_user(
    context: &mut Context,
    session: TypedSession,
    pool: &MySqlPool,
) -> Option<User> {
    match get_auth_user_entity(session, pool).await {
        Ok(auth_user_entity) => {
            let auth_user = models::translate_user(auth_user_entity);
            context.insert("auth_user", &auth_user);
            context.insert("is_auth", &true);
            Some(auth_user)
        }
        Err(UserContextError::NotAuthenticated) => {
            context.insert("is_auth", &false);
            None
        }
        Err(e) => {
            error!("Error getting authenticated user session: {:?}", e);
            context.insert("is_auth", &false);
            None
        }
    }
}
