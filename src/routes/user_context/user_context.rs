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

use super::{session_state::TypedSession, UserContextError};

const DEFAULT_HERO_BG_CLASS: &str = "hero-bg-landing";

pub struct UserContext {
    pub auth_user: Option<User>,
    pub context: Context,
    pub flash_messages: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Notifications {
    pub errors: Vec<String>,
    pub warns: Vec<String>,
    pub infos: Vec<String>,
    pub successes: Vec<String>,
}

pub fn get_empty(page_name: &str, image_path: Option<&str>) -> UserContext {
    let mut context = Context::new();
    insert_title(&mut context, page_name);
    context.insert(
        "notifications",
        &Notifications {
            errors: vec![],
            warns: vec![],
            infos: vec![],
            successes: vec![],
        },
    );
    context.insert("is_auth", &false);
    insert_hero_bg_class(&mut context, image_path);

    UserContext {
        auth_user: None,
        context,
        flash_messages: vec![],
    }
}

pub async fn build(
    session: TypedSession,
    flash_messages: IncomingFlashMessages,
    pool: &MySqlPool,
    page_name: &str,
    image_path: Option<&str>,
) -> UserContext {
    let mut context = Context::new();

    insert_title(&mut context, page_name);
    let flash_messages = insert_notifications(&mut context, flash_messages);
    let auth_user = insert_auth_user(&mut context, session, pool).await;
    insert_hero_bg_class(&mut context, image_path);

    UserContext {
        auth_user,
        context,
        flash_messages,
    }
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

fn insert_notifications(
    context: &mut Context,
    flash_messages: IncomingFlashMessages,
) -> Vec<String> {
    let mut errors: Vec<String> = vec![];
    let mut warns: Vec<String> = vec![];
    let mut infos: Vec<String> = vec![];
    let mut successes: Vec<String> = vec![];
    let mut debugs: Vec<String> = vec![];

    for m in flash_messages.iter() {
        let content = String::from(m.content());
        match m.level() {
            Level::Debug => debugs.push(content),
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

    debugs
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

fn insert_hero_bg_class(context: &mut Context, hero_bg_class: Option<&str>) {
    let hero_bg_class = match hero_bg_class {
        Some(h) => &h,
        None => DEFAULT_HERO_BG_CLASS,
    };

    context.insert("hero_bg_class", hero_bg_class);
}
