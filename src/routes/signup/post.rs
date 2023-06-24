use actix_web::{web, HttpResponse, Responder};
use secrecy::Secret;
use serde::Deserialize;
use sqlx::MySqlPool;

use crate::entities::user;

#[derive(Debug, Deserialize)]
pub struct SignupRequest {
    username: String,
    email: String,
    password: Secret<String>,
}

pub async fn process_signup(
    data: web::Form<SignupRequest>,
    pool: web::Data<MySqlPool>,
) -> impl Responder {
    println!(
        "Received signup request for: {} ({})",
        data.username, data.email
    );

    let result = user::create(
        &pool,
        &data.username,
        &data.email,
        &data.password,
    )
    .await;

    match result {
        Ok(user_id) =>
            HttpResponse::Ok()
                .body(format!("Successfully saved user: {}", user_id)),
        Err(err) =>
            HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error","message": format!("{:?}", err)}))
    }
}
