use async_recursion::async_recursion;
use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::entities::{
    comment::{Comment, CommentStore},
    content::ContentStore,
    user::UserStore,
    EntityError, EntityStores,
};

use super::{utils, UserModel};

pub const MAX_CHILD_COMMENTS: u8 = 8;
const MAX_DEPTH: usize = 8;

#[derive(Serialize)]
pub struct CommentModel {
    pub id: String,
    pub author: UserModel,
    pub created: DateTime<Utc>,
    pub created_pretty: String,
    pub content: String,
    pub children: Vec<CommentModel>,
}

#[async_recursion]
pub async fn translate_comment(
    stores: &EntityStores,
    comment: &Comment,
    depth: usize,
) -> Result<CommentModel, EntityError> {
    let author_entity = stores.user_store.get_by_id(comment.author_id).await?;
    let author = UserModel::from(author_entity);

    let children_entities = stores
        .comment_store
        .get_by_post_id_parent_id(comment.post_id, Some(comment.id), None, MAX_CHILD_COMMENTS)
        .await?;
    let mut children: Vec<CommentModel> = vec![];

    if depth < MAX_DEPTH {
        for child_entity in children_entities {
            let child_comment = translate_comment(stores, &child_entity, depth + 1).await?;
            children.push(child_comment);
        }
    }

    let content = stores.content_store.get_by_id(comment.content_id).await?;
    Ok(CommentModel {
        id: comment.public_id.clone(),
        author,
        created: comment.created,
        created_pretty: utils::get_readable_duration(comment.created),
        content: content.body_html,
        children,
    })
}
