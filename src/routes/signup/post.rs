use actix_web::{web, HttpResponse, Responder};
use actix_web_flash_messages::FlashMessage;
use secrecy::Secret;
use serde::Deserialize;
use sqlx::MySqlPool;

use crate::{
    entities::{
        user::{
            UserStore, MAX_PASSWORD_LENGTH, MAX_USERNAME_LENGTH, MIN_PASSWORD_LENGTH,
            MIN_USERNAME_LENGTH,
        },
        EntityError,
    },
    routes::{
        login::post::do_login_and_redirect, user_context::session_state::TypedSession, utils,
    },
};

#[derive(Debug, Deserialize)]
pub struct SignupRequest {
    username: String,
    email: String,
    password: Secret<String>,
}

pub async fn process_signup(
    session: TypedSession,
    pool: web::Data<MySqlPool>,
    data: web::Form<SignupRequest>,
    user_store: web::Data<dyn UserStore>,
) -> impl Responder {
    let result = user_store
        .insert(&data.username, &data.email, &data.password)
        .await;

    match result {
        Ok(_) => {
            FlashMessage::success("successfully signed up").send();
            do_login_and_redirect(session, user_store, &data.username, &data.password).await
        }
        Err(entity_error) => signup_error_redirect(entity_error),
    }
}

#[derive(Debug)]
pub enum SignupErrorCode {
    InvalidUsername,
    InvalidEmail,
    InvalidPassword,
    UserAlreadyExists,
    Unknown,
}

fn signup_error_redirect(entity_error: EntityError) -> HttpResponse {
    let error_code = match entity_error.to_owned() {
        EntityError::Internal(_) => return utils::redirect_entity_error(entity_error, "user"),
        EntityError::InvalidInput(param, _) => match param {
            "email" => SignupErrorCode::InvalidEmail,
            "name" => SignupErrorCode::InvalidUsername,
            "password" => SignupErrorCode::InvalidPassword,
            _ => SignupErrorCode::Unknown,
        },
        EntityError::MalformedData => SignupErrorCode::Unknown,
        EntityError::NotFound => SignupErrorCode::Unknown,
        EntityError::DuplicateKey => SignupErrorCode::UserAlreadyExists,
    };

    let error_message = get_error_message(error_code);

    utils::error_redirect("/signup", &error_message)
}

fn get_error_message(error_code: SignupErrorCode) -> String {
    let invalid_user = format!(
        "invalid username, min length {} characters, max length {} characters",
        MIN_USERNAME_LENGTH, MAX_USERNAME_LENGTH
    );
    let invalid_password = format!(
        "invalid password, min length {} characters, max length {} characters",
        MIN_PASSWORD_LENGTH, MAX_PASSWORD_LENGTH
    );

    let error_message = match error_code {
        SignupErrorCode::InvalidUsername => &invalid_user,
        SignupErrorCode::InvalidEmail => "invalid email, please enter a valid email address",
        SignupErrorCode::InvalidPassword => &invalid_password,
        SignupErrorCode::UserAlreadyExists => "username taken, try another",
        SignupErrorCode::Unknown => "an error has ocurred, please try again in a few minutes and/or contact the site administrator",
    };

    return error_message.to_owned();
}
