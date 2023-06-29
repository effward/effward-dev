use actix_web::{http::header::LOCATION, web, HttpResponse, Responder};
use actix_web_flash_messages::FlashMessage;
use log::{error, info};
use secrecy::Secret;
use serde::Deserialize;
use sqlx::MySqlPool;

use crate::{
    entities::{user, EntityError},
    routes::{models, session_state::TypedSession},
};

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    username: String,
    password: Secret<String>,
}

pub async fn process_login(
    session: TypedSession,
    data: web::Form<LoginRequest>,
    pool: web::Data<MySqlPool>,
) -> impl Responder {
    do_login_and_redirect(session, pool, &data.username, &data.password).await
}

pub async fn do_login_and_redirect(
    session: TypedSession,
    pool: web::Data<MySqlPool>,
    username: &String,
    password: &Secret<String>,
) -> HttpResponse {
    let result = user::get_by_name_password(&pool, username, password).await;

    match result {
        Ok(user_entity) => {
            let user = models::translate_user(user_entity);

            session.renew();
            match session.insert_user_id(user.id) {
                Ok(_) => {
                    info!("Successfully set user session");
                    FlashMessage::success("successfully logged in").send();
                    HttpResponse::SeeOther()
                        .insert_header((LOCATION, "/"))
                        .finish()
                }
                Err(e) => {
                    error!("Error inserting into session: {:?}", e);
                    login_error_redirect(LoginErrorCode::Unknown)
                }
            }
        }
        Err(entity_error) => {
            let error_code = convert_error(entity_error);
            login_error_redirect(error_code)
        }
    }
}

#[derive(Debug)]
enum LoginErrorCode {
    InvalidUsername,
    InvalidPassword,
    MalformedPassword,
    Unknown,
}

fn login_error_redirect(error_code: LoginErrorCode) -> HttpResponse {
    let error_message = get_error_message(error_code);
    FlashMessage::error(error_message).send();
    HttpResponse::SeeOther()
        .insert_header((LOCATION, "/login"))
        .finish()
}

fn convert_error(entity_error: EntityError) -> LoginErrorCode {
    match entity_error {
        EntityError::Internal(err) => {
            error!("🔥 internal error: {}", err);
            LoginErrorCode::Unknown
        }
        EntityError::InvalidInput(_, _) => LoginErrorCode::InvalidPassword,
        EntityError::MalformedData => LoginErrorCode::MalformedPassword,
        EntityError::NotFound => LoginErrorCode::InvalidUsername,
        EntityError::DuplicateKey => LoginErrorCode::Unknown,
    }
}

fn get_error_message(error_code: LoginErrorCode) -> String {
    let error_message = match error_code {
        LoginErrorCode::InvalidUsername => "incorrect username and/or password",
        LoginErrorCode::InvalidPassword => "incorrect password",
        LoginErrorCode::MalformedPassword => "password corrupted, please reset password below",
        LoginErrorCode::Unknown => "an error has ocurred, please try again in a few minutes and/or contact the site administrator",
    };

    return error_message.to_owned();
}
