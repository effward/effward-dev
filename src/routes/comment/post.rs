use actix_web::{web, Responder};
use serde::Deserialize;

use crate::{
    entities::{comment::CommentStore, post::PostStore, EntityStores},
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
    data: web::Form<CommentRequest>,
    stores: web::Data<EntityStores>,
) -> impl Responder {
    match user_context::get_auth_user_entity(session, &stores).await {
        Ok(auth_user_entity) => {
            let post = match stores.post_store.get_by_public_id(&data.post_id).await {
                Ok(p) => p,
                Err(entity_error) => {
                    return utils::redirect_entity_error(entity_error, "post");
                }
            };

            let parent_id = match data.parent_id.to_owned() {
                Some(parent_id) => match stores.comment_store.get_by_public_id(&parent_id).await {
                    Ok(p) => Some(p.id),
                    Err(entity_error) => {
                        return utils::redirect_entity_error(entity_error, "parent comment");
                    }
                },
                None => None,
            };

            match stores
                .comment_store
                .insert(&auth_user_entity.id, &post.id, &parent_id, &data.content)
                .await
            {
                Ok(_) => utils::success_redirect(
                    &format!("/post/{}", data.post_id),
                    "new comment successfully submitted, it should appear momentarily...",
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
