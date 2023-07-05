use actix_web::{web, HttpResponse, Responder};
use actix_web_flash_messages::IncomingFlashMessages;
use tera::Tera;

use crate::{
    entities::{post::PostStore, EntityStores},
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
    tera: web::Data<Tera>,
    path: web::Path<String>,
    stores: web::Data<EntityStores>,
) -> impl Responder {
    // TODO: handle errors
    let path_post = path.into_inner();
    let post = match stores.post_store.get_by_public_id(&path_post).await {
        Ok(p) => p,
        Err(entity_error) => {
            return utils::redirect_entity_error(entity_error, "post");
        }
    };

    let post_model = models::translate_post(&post, &stores).await.unwrap();

    let mut user_context = user_context::build(
        session,
        flash_messages,
        &stores,
        &format!("post - {}", post_model.summary.title),
        Some(HERO_BG_CLASS),
    )
    .await;

    user_context.context.insert("post", &post_model);

    // TODO: handle error
    let rendered = tera.render("post.html", &user_context.context).unwrap();

    HttpResponse::Ok().body(rendered)
}
