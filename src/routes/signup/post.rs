use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::MySqlPool;

use crate::entities::user;

#[derive(Debug, Deserialize)]
pub struct SignupRequest {
    username: String,
    email: String,
    password: String,
}

pub async fn process_signup(
    data: web::Form<SignupRequest>,
    pool: web::Data<MySqlPool>,
) -> impl Responder {
    println!(
        "Received signup request for: {} ({})",
        data.username, data.email
    );

    let user_id = user::create(
        &pool,
        data.username.clone(),
        data.email.clone(),
        data.password.clone(),
    )
    .await
    .unwrap();

    HttpResponse::Ok().body(format!("Successfully saved user: {}", user_id))
}
