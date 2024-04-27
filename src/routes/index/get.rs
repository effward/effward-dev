use actix_web::{web, HttpResponse, Responder};
use actix_web_flash_messages::{FlashMessage, IncomingFlashMessages};
use log::{debug, error};
use tera::Tera;

use crate::entities::EntityStores;
use crate::routes::user_context::{session_state::TypedSession, user_context};
use crate::{
    entities::post::{Post, PostStore},
    routes::models::{self, PostSummary},
};

const POSTS_PER_PAGE: u8 = 2;
const MAX_CONTENT_PREVIEW_LENGTH: usize = 1500;

pub async fn index(
    session: TypedSession,
    flash_messages: IncomingFlashMessages,
    tera: web::Data<Tera>,
    stores: web::Data<EntityStores>,
) -> impl Responder {
    debug!("getting user context");
    let mut user_context =
        user_context::build(session, flash_messages, &stores, "home", None).await;

    debug!("getting recent posts");
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
        match models::translate_post_summary(post_entity, &stores, MAX_CONTENT_PREVIEW_LENGTH).await {
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
