use actix_web::{web::Data, HttpResponse, Responder};
use actix_web_flash_messages::IncomingFlashMessages;
use tera::Tera;

use crate::{routes::user_context::{session_state::TypedSession, user_context}, entities::user::UserStore};

const HERO_BG_CLASS: &str = "hero-bg-signup";

pub async fn signup(
    session: TypedSession,
    flash_messages: IncomingFlashMessages,
    user_store: Data<dyn UserStore>,
    tera: Data<Tera>,
) -> impl Responder {
    let user_context = user_context::build(
        session,
        flash_messages,
        user_store,
        "signup",
        Some(HERO_BG_CLASS),
    )
    .await;

    // TODO: handle error
    let rendered = tera.render("signup.html", &user_context.context).unwrap();

    HttpResponse::Ok().body(rendered)
}
