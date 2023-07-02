use actix_web::{web, HttpResponse, Responder};
use actix_web_flash_messages::FlashMessage;
use log::error;
use secrecy::Secret;
use serde::Deserialize;
use sqlx::MySqlPool;

use crate::{
    entities::{
        user::{
            self, MAX_PASSWORD_LENGTH, MAX_USERNAME_LENGTH, MIN_PASSWORD_LENGTH,
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
) -> impl Responder {
    let result = user::insert(&pool, &data.username, &data.email, &data.password).await;

    match result {
        Ok(_) => {
            FlashMessage::success("successfully signed up").send();
            do_login_and_redirect(session, pool, &data.username, &data.password).await
        }
        Err(entity_error) => {
            let error_code = convert_error(entity_error);
            signup_error_redirect(error_code)
        }
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

fn signup_error_redirect(error_code: SignupErrorCode) -> HttpResponse {
    let error_message = get_error_message(error_code);
    FlashMessage::error(error_message).send();
    utils::redirect("/signup")
}

fn convert_error(entity_error: EntityError) -> SignupErrorCode {
    match entity_error {
        EntityError::Internal(err) => {
            error!("process_signup error: {}", err);
            SignupErrorCode::Unknown
        }
        EntityError::InvalidInput(param, _) => match param {
            "email" => SignupErrorCode::InvalidEmail,
            "name" => SignupErrorCode::InvalidUsername,
            "password" => SignupErrorCode::InvalidPassword,
            _ => SignupErrorCode::Unknown,
        },
        EntityError::MalformedData => SignupErrorCode::Unknown,
        EntityError::NotFound => SignupErrorCode::Unknown,
        EntityError::DuplicateKey => SignupErrorCode::UserAlreadyExists,
    }
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
