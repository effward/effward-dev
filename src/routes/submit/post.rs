use actix_web::{web, Responder};
use log::error;
use serde::Deserialize;
use sqlx::MySqlPool;

use crate::{
    entities::post,
    routes::{
<<<<<<< HEAD
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
    pool: web::Data<MySqlPool>,
    data: web::Form<SubmitRequest>,
) -> impl Responder {
    match user_context::get_auth_user_entity(session, &pool).await {
        Ok(auth_user_entity) => match post::insert(
            &pool,
            &auth_user_entity.id,
            &data.title,
            &data.link,
            &data.content,
        )
        .await
        {
            Ok(_post_id) => {
                // TODO: Redirect to post page (after creating post page)
<<<<<<< HEAD
                utils::success_redirect("/posts", "new post successfully submitted")
            },
            Err(entity_error) => {
                error!("Entity Error creating post: {:?}", entity_error);

                // TODO: preserve form contents on redirect so that submission isn't lost
                utils::warning_redirect("/submit", "something went wrong submitting your post, please try again")
            }
        },
        Err(e) => {
            error!("Error getting authenticated user: {:?}", e);
            utils::error_redirect("/login", "you must be logged in to submit posts")
        }
    }
}
