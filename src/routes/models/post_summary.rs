use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::entities::{
    comment::CommentStore, content::ContentStore, post::Post, user::UserStore, EntityError,
    EntityStores,
};

use super::{utils, UserModel};

#[derive(Serialize)]
pub struct PostSummary {
    pub id: String,
    pub author: UserModel,
    pub title: String,
    pub created: DateTime<Utc>,
    pub created_pretty: String,
    pub link: Option<String>,
    pub content: Option<String>,
    pub comment_count: i64,
}

pub async fn translate_post_summary(
    post: &Post,
    stores: &EntityStores,
) -> Result<PostSummary, EntityError> {
    let author_entity = stores.user_store.get_by_id(post.author_id).await?;
    let author = UserModel::from(author_entity);

    let content = match post.content_id {
        Some(id) => {
            let content = stores.content_store.get_by_id(id).await?;
            Some(content.body_html)
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
        comment_count,
    })
}
