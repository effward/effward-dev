use actix_web::{web, HttpResponse, Responder};
use tera::{Context, Tera};

pub async fn login(tera: web::Data<Tera>) -> impl Responder {
    println!("Login request");

    let mut data = Context::new();
    data.insert("title", "effward.dev - log in");

    // TODO: use unwrap_or_else() and handle error
    let rendered = tera.render("login.html", &data).unwrap();

    HttpResponse::Ok().body(rendered)
}
