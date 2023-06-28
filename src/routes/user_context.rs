use actix_web_flash_messages::{IncomingFlashMessages, Level};
use log::error;
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use tera::Context;

use crate::entities::user;

use super::{
    models::{self, User},
    session_state::TypedSession,
};

pub struct UserContext {
    pub user: Option<User>,
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

    context.insert("title", &format!("effward.dev - {}", page_name));

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

    let user_id = match session.get_user_id() {
        Ok(id) => id,
        Err(e) => {
            error!("Error fetching user session. Error: {:?}", e);
            None
        }
    };
    let user = match user_id {
        Some(user_id) => {
            let result = user::get_by_public_id(&pool, user_id).await;

            match result {
                Ok(user_entity) => {
                    let user = models::translate_user(user_entity);

                    Some(user)
                }
                Err(e) => {
                    error!("Error looking up user_id {}. Error: {:?}", user_id, e);
                    None
                }
            }
        }
        None => None,
    };

    match user {
        Some(u) => {
            context.insert("user", &u);
            context.insert("is_auth", &true);
            UserContext {
                user: Some(u),
                context,
            }
        }
        _ => {
            context.insert("is_auth", &false);
            UserContext {
                user: None,
                context,
            }
        }
    }
}
