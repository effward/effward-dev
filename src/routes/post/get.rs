use actix_web::{web, HttpResponse, Responder};
use actix_web_flash_messages::IncomingFlashMessages;
use shortguid::ShortGuid;
use sqlx::MySqlPool;
use tera::Tera;

use crate::{
    entities::post,
    routes::{
        models,
        user_context::{session_state::TypedSession, user_context},
        utils,
    },
};

const HERO_BG_CLASS: &str = "hero-bg-post";

pub async fn post(
    session: TypedSession,
    flash_messages: IncomingFlashMessages,
    pool: web::Data<MySqlPool>,
    tera: web::Data<Tera>,
    path: web::Path<String>,
) -> impl Responder {
    // TODO: handle errors
    let path_post = path.into_inner();
    let post_entity = match ShortGuid::try_parse(&path_post) {
        Ok(post_id) => match post::get_by_public_id(&pool, *post_id.as_uuid()).await {
            Ok(p) => p,
            Err(entity_error) => {
                return utils::redirect_entity_error(entity_error, "post");
            }
        },
        Err(_) => {
            return utils::error_redirect("/error/404", "invalid post uuid");
        }
    };

    let post = models::translate_post(&pool, &post_entity).await.unwrap();

    let mut user_context = user_context::build(
        session,
        flash_messages,
        &pool,
        &format!("post - {}", post.summary.title),
        Some(HERO_BG_CLASS),
    )
    .await;

    user_context.context.insert("post", &post);

    // TODO: handle error
    let rendered = tera.render("post.html", &user_context.context).unwrap();

    HttpResponse::Ok().body(rendered)
}
