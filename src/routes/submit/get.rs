use actix_web::{web::Data, HttpResponse, Responder};
use actix_web_flash_messages::{FlashMessage, IncomingFlashMessages};
use tera::Tera;

use crate::{
    entities::{user::UserStore, EntityStores},
    routes::{
        user_context::{session_state::TypedSession, user_context},
        utils,
    },
};

const HERO_BG_CLASS: &str = "hero-bg-submit";

pub async fn submit(
    session: TypedSession,
    flash_messages: IncomingFlashMessages,
    stores: Data<EntityStores>,
    tera: Data<Tera>,
) -> impl Responder {
    let user_context = user_context::build(
        session,
        flash_messages,
        &stores,
        "submit",
        Some(HERO_BG_CLASS),
    )
    .await;

    match user_context.auth_user {
        Some(_) => {
            // TODO: handle error
            let rendered = tera.render("submit.html", &user_context.context).unwrap();
            HttpResponse::Ok().body(rendered)
        }
        None => {
            FlashMessage::warning("you must be logged in to submit posts").send();
            utils::redirect("/login")
        }
    }
}
