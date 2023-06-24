use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::MySqlPool;

use crate::{entities::user, routes::models};

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

pub async fn process_login(
    data: web::Form<LoginRequest>,
    pool: web::Data<MySqlPool>,
) -> impl Responder {
    println!("Received login request for: {}", data.username);

    let result =
        user::get_by_name_password(&pool, data.username.clone(), data.password.clone()).await;

    match result {
        Ok(user) => {
            println!("Found User in DB");
            let user_response = serde_json::json!({"status": "success", "data": serde_json::json!({
                    "user": models::translate_user(&user)
            })});

            return HttpResponse::Ok().json(user_response);
        }
        Err(err) => match err {
            user::UserError::MalformedPassword => {
                return HttpResponse::InternalServerError()
                    .json(serde_json::json!({"status": "error","message": "Password malformed."}));
            }
            user::UserError::InvalidPassword => {
                return HttpResponse::Unauthorized().json(
                    serde_json::json!({"status": "error", "message": "Incorrect password."}),
                );
            }
            user::UserError::NotFound => {
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
