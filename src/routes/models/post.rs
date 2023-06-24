use serde::Serialize;

#[derive(Serialize)]
pub struct Post {
    pub author: String,
    pub title: String,
    pub body: String,
}
