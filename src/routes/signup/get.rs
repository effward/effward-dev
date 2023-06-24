use actix_web::{web, HttpResponse, Responder};
use tera::{Context, Tera};

pub async fn signup(tera: web::Data<Tera>) -> impl Responder {
    println!("Signup request");

    let mut data = Context::new();
    data.insert("title", "effward.dev - sign up");

    // TODO: use unwrap_or_else() and handle error
    let rendered = tera.render("signup.html", &data).unwrap();

    HttpResponse::Ok().body(rendered)
}
