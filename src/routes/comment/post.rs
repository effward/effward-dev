use actix_web::{web, Responder};
use serde::Deserialize;
use shortguid::ShortGuid;
use sqlx::MySqlPool;

use crate::{
    entities::{comment, post},
    routes::{
        user_context::{session_state::TypedSession, user_context, UserContextError},
        utils,
    },
};

#[derive(Debug, Deserialize)]
pub struct CommentRequest {
    post_id: String,
    parent_id: Option<String>,
    content: String,
}

pub async fn process_comment(
    session: TypedSession,
    pool: web::Data<MySqlPool>,
    data: web::Form<CommentRequest>,
) -> impl Responder {
    match user_context::get_auth_user_entity(session, &pool).await {
        Ok(auth_user_entity) => {
            let post_entity = match ShortGuid::try_parse(&data.post_id) {
                Ok(post_public_id) => {
                    match post::get_by_public_id(&pool, *post_public_id.as_uuid()).await {
                        Ok(p) => p,
                        Err(entity_error) => {
                            return utils::redirect_entity_error(entity_error, "post");
                        }
                    }
                }
                Err(_) => {
                    return utils::error_redirect("/error/404", "invalid post uuid");
                }
            };

            let parent_id = match data.parent_id.to_owned() {
                Some(parent_id) => match ShortGuid::try_parse(parent_id) {
                    Ok(parent_public_id) => {
                        match comment::get_by_public_id(&pool, *parent_public_id.as_uuid()).await {
                            Ok(p) => Some(p.id),
                            Err(entity_error) => {
                                return utils::redirect_entity_error(
                                    entity_error,
                                    "parent comment",
                                );
                            }
                        }
                    }
                    Err(_) => {
                        return utils::error_redirect("/error/404", "invalid parent comment uuid");
                    }
                },
                None => None,
            };

            match comment::insert(
                &pool,
                &auth_user_entity.id,
                &post_entity.id,
                &parent_id,
                &data.content,
            )
            .await
            {
                Ok(_) => utils::success_redirect(
                    &format!("/post/{}", data.post_id),
                    "new comment successfully submitted",
                ),
                Err(_) => utils::warning_redirect(
                    &format!("/post/{}", data.post_id),
                    "something went wrong submitting your comment, please try again",
                ),
            }
        }
        Err(e) => match e {
            UserContextError::SessionStore(_) => {
                utils::error_redirect("/login", "error getting user session, please log in again")
            }
            UserContextError::UuidParsing(_) => {
                utils::error_redirect("/login", "error parsing user session, please log in again")
            }
            UserContextError::EntityError(e) => utils::redirect_entity_error(e, "comment"),
            UserContextError::NotAuthenticated => {
                utils::error_redirect("/login", "you must be logged in to submit comments")
            }
        },
    }
}
