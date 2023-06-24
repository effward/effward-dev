use actix_web::{web, HttpResponse, Responder};
use tera::{Context, Tera};

pub async fn submit(tera: web::Data<Tera>) -> impl Responder {
    println!("Submit request");

    let mut data = Context::new();
    data.insert("title", "effward.dev - submit");

    let rendered = tera.render("submit.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}
