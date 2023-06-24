use actix_web::{web, HttpResponse, Responder};
use tera::{Context, Tera};

use crate::routes::models::Post;

pub async fn home(tera: web::Data<Tera>) -> impl Responder {
    println!("Home request");

    let mut data = Context::new();

    let posts = [
        Post {
            title: String::from("First Post"),
            body: String::from("Body content"),
            author: String::from("effward"),
        },
        Post {
            title: String::from("Second Post"),
            body: String::from("Second body content!"),
            author: String::from("beeward"),
        },
        Post {
            title: String::from("Third Post"),
            body: String::from("Third body content?"),
            author: String::from("effward"),
        },
    ];

    data.insert("title", "effward.dev - home");
    data.insert("name", "effward");
    data.insert("posts", &posts);

    // TODO: use unwrap_or_else() and handle error
    let rendered = tera.render("index.html", &data).unwrap();

    HttpResponse::Ok().body(rendered)
}
