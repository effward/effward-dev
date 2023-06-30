use actix_web::{web, HttpResponse, Responder};
use actix_web_flash_messages::{FlashMessage, IncomingFlashMessages};
use log::error;
use sqlx::MySqlPool;
use tera::Tera;

use crate::{
    entities::{self, post::PostEntity},
    routes::{
        models::{self, PostSummary},
        user_context::{user_context::build_user_context, TypedSession},
    },
};

const POSTS_PER_PAGE: u8 = 15;

pub async fn posts(
    session: TypedSession,
    flash_message: IncomingFlashMessages,
    pool: web::Data<MySqlPool>,
    tera: web::Data<Tera>,
) -> impl Responder {
    let mut user_context = build_user_context(session, flash_message, &pool, "posts").await;

    let result = entities::post::get_recent(&pool, None, POSTS_PER_PAGE).await;

    let post_entities = match result {
        Ok(p) => p,
        Err(e) => {
            error!("Error fetching recent posts: {:?}", e);
            FlashMessage::error("error fetching recent posts, try again in a few").send();
            Vec::<PostEntity>::new()
        }
    };

    let mut posts: Vec<PostSummary> = vec![];
    for post_entity in post_entities.iter() {
        match models::translate_post_summary(&pool, post_entity).await {
            Ok(post_summary) => {
                posts.push(post_summary);
            }
            Err(e) => {
                error!("Error translating post: {:?}", e);
                FlashMessage::error("error loading post").send();
            }
        };
    }

    user_context.context.insert("posts", &posts);

    // TODO: handle error
    let rendered = tera.render("posts.html", &user_context.context).unwrap();

    HttpResponse::Ok().body(rendered)
}