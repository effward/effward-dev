use actix_web::{web, HttpResponse, Responder};
use actix_web_flash_messages::IncomingFlashMessages;
use tera::Tera;

use crate::{routes::user_context::{session_state::TypedSession, user_context}, entities::user::UserStore};

const HERO_BG_CLASS: &str = "hero-bg-login";

pub async fn login(
    session: TypedSession,
    flash_messages: IncomingFlashMessages,
    user_store: web::Data<dyn UserStore>,
    tera: web::Data<Tera>,
) -> impl Responder {
    let user_context =
        user_context::build(session, flash_messages, user_store, "login", Some(HERO_BG_CLASS)).await;

    // TODO: handle error
    let rendered = tera.render("login.html", &user_context.context).unwrap();

    HttpResponse::Ok().body(rendered)
}
