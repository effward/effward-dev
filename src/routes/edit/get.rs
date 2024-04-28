use actix_web::{web::Data, HttpResponse, Responder};
use actix_web_flash_messages::{FlashMessage, IncomingFlashMessages};
use tera::Tera;

use crate::{
    entities::EntityStores,
    routes::{
        user_context::{session_state::TypedSession, user_context},
        utils,
    },
};

const HERO_BG_CLASS: &str = "hero-bg-submit";

pub async fn edit(
    session: TypedSession,
    flash_messages: IncomingFlashMessages,
    stores: Data<EntityStores>,
    tera: Data<Tera>,
    path: web::Path<String>,
) -> impl Responder {
    // TODO: handle errors
    let public_id = path.into_inner();
    let post = match stores.post_store.get_by_public_id(&public_id).await {
        Ok(p) => p,
        Err(entity_error) => {
            return utils::redirect_entity_error(entity_error, "post");
        }
    };

    let post_model = models::translate_post(&post, &stores).await.unwrap();
    
    let user_context = user_context::build(
        session,
        flash_messages,
        &stores,
        &format!("edit post - {}", post_model.summary.title),
        Some(HERO_BG_CLASS),
    )
    .await;
    
    user_context.context.insert("post", &post_model);

    match user_context.auth_user {
        Some(_) => {
            // TODO: handle error
            let rendered = tera.render("edit.html", &user_context.context).unwrap();
            HttpResponse::Ok().body(rendered)
        }
        None => {
            FlashMessage::warning("you must be logged in to submit posts").send();
            utils::redirect("/login")
        }
    }
}
