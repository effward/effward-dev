use actix_web::{web, HttpResponse, Responder};
use actix_web_flash_messages::FlashMessage;
use log::{error, info};
use secrecy::Secret;
use serde::Deserialize;

use crate::{
    entities::{user::UserStore, EntityError, EntityStores},
    routes::{models::UserModel, user_context::session_state::TypedSession, utils},
};

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    username: String,
    password: Secret<String>,
}

pub async fn process_login(
    session: TypedSession,
    data: web::Form<LoginRequest>,
    stores: web::Data<EntityStores>,
) -> impl Responder {
    do_login_and_redirect(session, &stores, &data.username, &data.password).await
}

pub async fn do_login_and_redirect(
    session: TypedSession,
    stores: &EntityStores,
    username: &String,
    password: &Secret<String>,
) -> HttpResponse {
    let result = stores
        .user_store
        .get_by_name_password(username, password)
        .await;

    match result {
        Ok(user) => {
            let user_model = UserModel::from(user);

            session.renew();
            match session.insert_user_id(user_model.id) {
                Ok(_) => {
                    info!("Successfully set user session");
                    FlashMessage::success("successfully logged in").send();
                    utils::redirect("/")
                }
                Err(e) => {
                    error!("Error inserting into session: {:?}", e);
                    login_error_redirect(EntityError::Internal(e.to_string()))
                }
            }
        }
        Err(entity_error) => login_error_redirect(entity_error),
    }
}

#[derive(Debug)]
enum LoginErrorCode {
    InvalidUsername,
    InvalidPassword,
    MalformedPassword,
    Unknown,
}

fn login_error_redirect(entity_error: EntityError) -> HttpResponse {
    let error_code = match entity_error.to_owned() {
        EntityError::Internal(_) => return utils::redirect_entity_error(entity_error, "user"),
        EntityError::InvalidInput(_, _) => LoginErrorCode::InvalidPassword,
        EntityError::MalformedData => LoginErrorCode::MalformedPassword,
        EntityError::NotFound => LoginErrorCode::InvalidUsername,
        EntityError::DuplicateKey => LoginErrorCode::Unknown,
    };

    let error_message = get_error_message(error_code);

    utils::error_redirect("/login", error_message)
}

fn get_error_message(error_code: LoginErrorCode) -> &'static str {
    match error_code {
        LoginErrorCode::InvalidUsername => "incorrect username and/or password",
        LoginErrorCode::InvalidPassword => "incorrect password",
        LoginErrorCode::MalformedPassword => "password corrupted, please reset password below",
        LoginErrorCode::Unknown => "an error has ocurred, please try again in a few minutes and/or contact the site administrator",
    }
}
