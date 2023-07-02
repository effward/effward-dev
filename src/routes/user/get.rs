use actix_web::{web, HttpResponse, Responder};
use actix_web_flash_messages::IncomingFlashMessages;
use shortguid::ShortGuid;
use sqlx::MySqlPool;
use tera::Tera;
use uuid::Uuid;

use crate::{
    entities::user,
    routes::{
        models,
        user_context::{session_state::TypedSession, user_context},
        utils,
    },
};

pub async fn user(
    session: TypedSession,
    flash_messages: IncomingFlashMessages,
    pool: web::Data<MySqlPool>,
    tera: web::Data<Tera>,
    path: web::Path<String>,
) -> impl Responder {
    // TODO: handle errors
    let path_user = path.into_inner();
    let user_entity = match ShortGuid::try_parse(&path_user) {
        Ok(user_id) => user::get_by_public_id(&pool, *user_id.as_uuid())
            .await
            .unwrap(),
        Err(_) => match Uuid::try_parse(&path_user) {
            Ok(user_id) => user::get_by_public_id(&pool, user_id).await.unwrap(),
            Err(_) => match user::get_by_name(&pool, &path_user).await {
                Ok(u) => u,
                Err(e) => {
                    return utils::redirect_entity_error(e, "user");
                }
            },
        },
    };

    let user = models::translate_user(user_entity);

    let mut user_context = user_context::build(
        session,
        flash_messages,
        &pool,
        &format!("user - {}", user.name),
        None,
    )
    .await;

    user_context.context.insert("user", &user);

    // TODO: handle error
    let rendered = tera.render("user.html", &user_context.context).unwrap();

    HttpResponse::Ok().body(rendered)
}
