use actix_web::{web, HttpResponse, Responder};
use actix_web_flash_messages::IncomingFlashMessages;
use sqlx::MySqlPool;
use tera::Tera;

use crate::routes::user_context::{session_state::TypedSession, user_context};

const HERO_BG_CLASS: &str = "hero-bg-login";

pub async fn login(
    session: TypedSession,
    flash_messages: IncomingFlashMessages,
    pool: web::Data<MySqlPool>,
    tera: web::Data<Tera>,
) -> impl Responder {
    let user_context =
        user_context::build(session, flash_messages, &pool, "login", Some(HERO_BG_CLASS)).await;

    // TODO: handle error
    let rendered = tera.render("login.html", &user_context.context).unwrap();

    HttpResponse::Ok().body(rendered)
}
