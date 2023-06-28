use actix_web::{web, HttpResponse, Responder};
use actix_web_flash_messages::IncomingFlashMessages;
use sqlx::MySqlPool;
use tera::Tera;

use crate::routes::{session_state::TypedSession, user_context::build_user_context};

pub async fn login(
    session: TypedSession,
    flash_messages: IncomingFlashMessages,
    pool: web::Data<MySqlPool>,
    tera: web::Data<Tera>,
) -> impl Responder {
    let user_context = build_user_context(session, flash_messages, &pool, "login").await;

    // TODO: handle error
    let rendered = tera.render("login.html", &user_context.context).unwrap();

    HttpResponse::Ok().body(rendered)
}
