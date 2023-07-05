use actix_web::web::Data;
use chrono::{NaiveDateTime, Utc, DateTime};
use serde::Serialize;
use sqlx::MySqlPool;
use std::cmp;
use substring::Substring;

use crate::entities::{
    comment, content, post::Post, user::UserStore, EntityError, EntityStores,
};

use super::{utils, UserModel};

const POST_PREVIEW_LENGTH: usize = 250;

#[derive(Serialize)]
pub struct PostSummary {
    pub id: String,
    pub author: UserModel,
    pub title: String,
    pub created: DateTime<Utc>,
    pub created_pretty: String,
    pub link: Option<String>,
    pub content: Option<String>,
    pub post_preview: Option<String>,
    pub comment_count: i64,
}

pub async fn translate_post_summary(
    pool: &MySqlPool,
    post: &Post,
    stores: &EntityStores,
) -> Result<PostSummary, EntityError> {
    let author_entity = stores.user_store.get_by_id(post.author_id).await?;
    let author = UserModel::from(author_entity);

    let mut post_preview: Option<String> = None;
    let content = match post.content_id {
        Some(id) => {
            let content = content::get_by_id(pool, id).await?;
            let preview_length = cmp::min(content.body.len(), POST_PREVIEW_LENGTH);
            let mut preview = content.body.substring(0, preview_length).to_owned();

            if content.body.len() > POST_PREVIEW_LENGTH {
                preview.push_str("…");
            }
            post_preview = Some(preview);
            Some(content.body)
        }
        None => None,
    };

    let comment_count = comment::get_count_by_post_id(pool, &post.id).await?;

    Ok(PostSummary {
        id: post.public_id.clone(),
        author,
        title: post.title.to_owned(),
        created: post.created,
        created_pretty: utils::get_readable_duration(post.created),
        link: post.link.to_owned(),
        content,
        post_preview,
        comment_count,
    })
}
