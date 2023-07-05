use chrono::{Utc, DateTime};
use serde::Serialize;
use std::cmp;
use substring::Substring;

use crate::entities::{
    comment::{self, CommentStore}, content::ContentStore, post::Post, user::UserStore, EntityError, EntityStores,
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
    post: &Post,
    stores: &EntityStores,
) -> Result<PostSummary, EntityError> {
    let author_entity = stores.user_store.get_by_id(post.author_id).await?;
    let author = UserModel::from(author_entity);

    let mut post_preview: Option<String> = None;
    let content = match post.content_id {
        Some(id) => {
            let content = stores.content_store.get_by_id(id).await?;
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

    let comment_count = stores.comment_store.get_count_by_post_id(&post.id).await?;

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
