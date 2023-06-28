use actix_web::{web, HttpResponse, Responder};
use actix_web_flash_messages::IncomingFlashMessages;
use sqlx::MySqlPool;
use tera::Tera;

use crate::routes::{models::Post, session_state::TypedSession, user_context::build_user_context};

pub async fn home(
    session: TypedSession,
    flash_message: IncomingFlashMessages,
    pool: web::Data<MySqlPool>,
    tera: web::Data<Tera>,
) -> impl Responder {
    let user_context = build_user_context(session, flash_message, &pool, "home").await;

    let posts = [
        Post {
            title: String::from("First Post"),
            body: String::from("Body content"),
            author: String::from("effward"),
        },
        Post {
            title: String::from("Second Post"),
            body: String::from("Second body content!"),
            author: String::from("beeward"),
        },
        Post {
            title: String::from("Third Post"),
            body: String::from("Third body content?"),
            author: String::from("effward"),
        },
    ];

    let name = match user_context.user {
        Some(user) => user.name,
        None => String::from("stranger"),
    };

    let mut context = user_context.context;
    context.insert("name", &name);
    context.insert("posts", &posts);

    // TODO: handle error
    let rendered = tera.render("index.html", &context).unwrap();

    HttpResponse::Ok().body(rendered)
}
