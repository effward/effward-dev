use actix_web::{web, HttpResponse, Responder};
use actix_web_flash_messages::IncomingFlashMessages;
use tera::Tera;

use crate::{
    entities::EntityStores,
    routes::user_context::{session_state::TypedSession, user_context},
};

const HERO_BG_CLASS: &str = "hero-bg-login";

pub async fn login(
    session: TypedSession,
    flash_messages: IncomingFlashMessages,
    stores: web::Data<EntityStores>,
    tera: web::Data<Tera>,
) -> impl Responder {
    let user_context = user_context::build(
        session,
        flash_messages,
        &stores,
        "login",
        Some(HERO_BG_CLASS),
    )
    .await;

    // TODO: handle error
    let rendered = tera.render("login.html", &user_context.context).unwrap();

    HttpResponse::Ok().body(rendered)
}
