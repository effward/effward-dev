use actix_web::{web, HttpResponse, Responder};
use actix_web_flash_messages::{FlashMessage, IncomingFlashMessages};
use sqlx::MySqlPool;
use tera::Tera;

use crate::routes::{
    user_context::{user_context::build_user_context, TypedSession},
    utils,
};

pub async fn submit(
    session: TypedSession,
    flash_messages: IncomingFlashMessages,
    pool: web::Data<MySqlPool>,
    tera: web::Data<Tera>,
) -> impl Responder {
    let user_context = build_user_context(session, flash_messages, &pool, "submit").await;

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
