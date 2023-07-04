use actix_web::{web, HttpResponse, Responder};
use actix_web_flash_messages::IncomingFlashMessages;
use sqlx::MySqlPool;
use tera::Tera;

use crate::{
    entities::user::UserStore,
    routes::{
        models::UserModel,
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
    user_store: web::Data<dyn UserStore>,
) -> impl Responder {
    // TODO: handle errors
    let path_user = path.into_inner();
    let user = match user_store.get_by_public_id(&path_user).await {
        Ok(u) => u,
        Err(entity_error) => {
            return utils::redirect_entity_error(entity_error, "user");
        }
    };

    let user_model = UserModel::from(user);

    let mut user_context = user_context::build(
        session,
        flash_messages,
        user_store,
        &format!("user - {}", user_model.name),
        None,
    )
    .await;

    user_context.context.insert("user", &user_model);

    // TODO: handle error
    let rendered = tera.render("user.html", &user_context.context).unwrap();

    HttpResponse::Ok().body(rendered)
}
