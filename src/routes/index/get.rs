use actix_web::{web, HttpResponse, Responder};
use actix_web_flash_messages::{FlashMessage, IncomingFlashMessages};
use log::{error, debug};
use sqlx::MySqlPool;
use tera::Tera;

use crate::entities::user::UserStore;
use crate::routes::user_context::{session_state::TypedSession, user_context};
use crate::{
    entities::{self, post::PostEntity},
    routes::models::{self, PostSummary},
};

const POSTS_PER_PAGE: u8 = 2;

pub async fn index(
    session: TypedSession,
    flash_messages: IncomingFlashMessages,
    pool: web::Data<MySqlPool>,
    tera: web::Data<Tera>,
    user_store: web::Data<dyn UserStore>,
) -> impl Responder {
    debug!("getting user context");
    let mut user_context = user_context::build(session, flash_messages, user_store.clone(), "home", None).await;

    debug!("getting recent posts");
    let result = entities::post::get_recent(&pool, None, POSTS_PER_PAGE).await;

    let post_entities = match result {
        Ok(p) => p,
        Err(e) => {
            error!("Error fetching recent posts: {:?}", e);
            FlashMessage::error("error fetching recent posts, try again in a few").send();
            Vec::<PostEntity>::new()
        }
    };

    let mut posts: Vec<PostSummary> = vec![];
    for post_entity in post_entities.iter() {
        match models::translate_post_summary(&pool, post_entity, user_store.clone()).await {
            Ok(post_summary) => {
                posts.push(post_summary);
            }
            Err(e) => {
                error!("Error translating post: {:?}", e);
                FlashMessage::error("error loading post").send();
            }
        };
    }

    let name = match user_context.auth_user {
        Some(auth_user) => auth_user.name,
        None => String::from("stranger"),
    };

    user_context.context.insert("name", &name);
    user_context.context.insert("posts", &posts);

    // TODO: handle error
    let rendered = tera.render("index.html", &user_context.context).unwrap();

    HttpResponse::Ok().body(rendered)
}
