use actix_web::{
    web::{Data, Form},
    Responder,
};
use log::error;
use serde::Deserialize;

use crate::{
    entities::{post::PostStore, EntityStores},
    routes::{
        user_context::{session_state::TypedSession, user_context},
        utils,
    },
};

#[derive(Debug, Deserialize)]
pub struct SubmitRequest {
    title: String,
    link: Option<String>,
    content: Option<String>,
}

pub async fn process_submission(
    session: TypedSession,
    data: Form<SubmitRequest>,
    stores: Data<EntityStores>,
) -> impl Responder {
    match user_context::get_auth_user_entity(session, &stores).await {
        Ok(auth_user_entity) => match stores
            .post_store
            .insert(&auth_user_entity.id, &data.title, &data.link, &data.content)
            .await
        {
            Ok(post) => utils::success_redirect(
                &format!("/post/{}", post.public_id),
                "new post successfully submitted, it should appear momentarily...",
            ),
            Err(entity_error) => {
                error!("Entity Error creating post: {:?}", entity_error);

                // TODO: preserve form contents on redirect so that submission isn't lost
                utils::warning_redirect(
                    "/submit",
                    "something went wrong submitting your post, please try again",
                )
            }
        },
        Err(e) => {
            error!("Error getting authenticated user: {:?}", e);
            utils::error_redirect("/login", "you must be logged in to submit posts")
        }
    }
}
