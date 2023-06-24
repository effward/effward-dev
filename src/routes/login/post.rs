use actix_web::{web, HttpResponse, Responder, http::header::LOCATION};
use secrecy::Secret;
use serde::Deserialize;
use sqlx::MySqlPool;

use crate::entities::{user, EntityError};

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    username: String,
    password: Secret<String>,
}

pub async fn process_login(
    data: web::Form<LoginRequest>,
    pool: web::Data<MySqlPool>,
) -> impl Responder {
    println!("Received login request for: {}", data.username);

    let result =
        user::get_by_name_password(&pool, &data.username, &data.password).await;

    match result {
        Ok(_) => {
            println!("Found User in DB");

            return HttpResponse::SeeOther()
                .insert_header((LOCATION, "/"))
                .finish();
        }
        Err(err) => match err {
            EntityError::MalformedData => {
                return HttpResponse::InternalServerError()
                    .json(serde_json::json!({"status": "error","message": "Password malformed."}));
            }
            EntityError::InvalidPassword => {
                return HttpResponse::Unauthorized().json(
                    serde_json::json!({"status": "error", "message": "Incorrect password."}),
                );
            }
            EntityError::NotFound => {
                return HttpResponse::NotFound().json(
                    serde_json::json!({"status": "error", "message": "User doesn't exist."}),
                );
            }
            _ => {
                return HttpResponse::InternalServerError()
                    .json(serde_json::json!({"status": "error","message": format!("{:?}", err)}));
            }
        },
    }
}
