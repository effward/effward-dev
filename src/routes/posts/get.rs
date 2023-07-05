use actix_web::{web, HttpResponse, Responder};
use actix_web_flash_messages::{FlashMessage, IncomingFlashMessages};
use log::error;
use tera::Tera;

use crate::{
    entities::{
        post::{Post, PostStore},
        EntityStores,
    },
    routes::{
        models::{self, PostSummary},
        user_context::{session_state::TypedSession, user_context},
    },
};

const POSTS_PER_PAGE: u8 = 15;
const HERO_BG_CLASS: &str = "hero-bg-posts";

pub async fn posts(
    session: TypedSession,
    flash_messages: IncomingFlashMessages,
    tera: web::Data<Tera>,
    stores: web::Data<EntityStores>,
) -> impl Responder {
    let mut user_context = user_context::build(
        session,
        flash_messages,
        &stores,
        "posts",
        Some(HERO_BG_CLASS),
    )
    .await;

    let result = stores.post_store.get_recent(None, POSTS_PER_PAGE).await;

    let post_entities = match result {
        Ok(p) => p,
        Err(e) => {
            error!("Error fetching recent posts: {:?}", e);
            FlashMessage::error("error fetching recent posts, try again in a few").send();
            Vec::<Post>::new()
        }
    };

    let mut posts: Vec<PostSummary> = vec![];
    for post_entity in post_entities.iter() {
        match models::translate_post_summary(post_entity, &stores).await {
            Ok(post_summary) => {
                posts.push(post_summary);
            }
            Err(e) => {
                error!("Error translating post: {:?}", e);
                FlashMessage::error("error loading post").send();
            }
        };
    }

    user_context.context.insert("posts", &posts);

    // TODO: handle error
    let rendered = tera.render("posts.html", &user_context.context).unwrap();

    HttpResponse::Ok().body(rendered)
}
