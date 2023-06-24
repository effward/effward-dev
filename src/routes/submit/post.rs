use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Submission {
    title: String,
    _body: String,
}

pub async fn process_submission(data: web::Form<Submission>) -> impl Responder {
    println!("{:?}", data);

    HttpResponse::Ok().body(format!("Posted submission: {}", data.title))
}
