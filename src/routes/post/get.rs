use actix_web::{web, HttpResponse, Responder};
use actix_web_flash_messages::IncomingFlashMessages;
use shortguid::ShortGuid;
use sqlx::MySqlPool;
use tera::Tera;
use uuid::Uuid;

use crate::{
    entities::post,
    routes::{
        models,
        user_context::{user_context::build_user_context, TypedSession},
    },
};

pub async fn post(
    session: TypedSession,
    flash_message: IncomingFlashMessages,
    pool: web::Data<MySqlPool>,
    tera: web::Data<Tera>,
    path: web::Path<String>,
) -> impl Responder {
    // TODO: handle errors
    let path_post = path.into_inner();
    let post_entity = match ShortGuid::try_parse(&path_post) {
        Ok(post_id) => post::get_by_public_id(&pool, *post_id.as_uuid())
            .await
            .unwrap(),
        Err(_) => {
            let post_id = Uuid::try_parse(&path_post).unwrap();
            post::get_by_public_id(&pool, post_id).await.unwrap()
        }
    };

    let post = models::translate_post_summary(&pool, &post_entity)
        .await
        .unwrap();

    let mut user_context = build_user_context(
        session,
        flash_message,
        &pool,
        &format!("post - {}", post.title),
    )
    .await;

    user_context.context.insert("post", &post);

    // TODO: handle error
    let rendered = tera.render("post.html", &user_context.context).unwrap();

    HttpResponse::Ok().body(rendered)
}