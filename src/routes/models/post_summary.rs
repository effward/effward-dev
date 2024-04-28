use chrono::{DateTime, Utc};
use serde::Serialize;
use substring::Substring;

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
    pub content_raw: Option<String>,
    pub comment_count: i64,
}

pub async fn translate_post_summary(
    post: &Post,
    stores: &EntityStores,
    max_content_len: usize,
) -> Result<PostSummary, EntityError> {
    let author_entity = stores.user_store.get_by_id(post.author_id).await?;
    let author = UserModel::from(author_entity);

    let content_entity = match post.content_id {
        Some(id) => {
            let content = stores.content_store.get_by_id(id).await?;
            Some(content)
        }
        None => None,
    };

    let (content, content_raw) = match content_entity {
        Some(ce) => {
            let mut html = ce.body_html;
            if max_content_len > 0 && html.len() > max_content_len {
                html = html.substring(0, max_content_len).to_string();
                let elipses = &format!("<a href='/post/{}'>... (see more)</a>", post.public_id);
                html.push_str(elipses);
            }
            (Some(html), Some(ce.body))
        }
        None => (None, None),
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
        content_raw,
        comment_count,
    })
}
