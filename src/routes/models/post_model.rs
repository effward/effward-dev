use serde::Serialize;

use crate::entities::comment::CommentStore;
use crate::entities::{post::Post, EntityError, EntityStores};

use super::{comment::translate_comment, translate_post_summary, CommentModel, PostSummary};

pub const MAX_TOP_LEVEL_COMMENTS: u8 = 50;

#[derive(Serialize)]
pub struct PostModel {
    pub summary: PostSummary,
    pub comments: Vec<CommentModel>,
}

pub async fn translate_post(post: &Post, stores: &EntityStores) -> Result<PostModel, EntityError> {
    let summary = translate_post_summary(post, stores, 0).await?;

    let comment_entities = stores
        .comment_store
        .get_by_post_id_parent_id(post.id, None, None, MAX_TOP_LEVEL_COMMENTS)
        .await?;

    let mut comments: Vec<CommentModel> = vec![];
    for comment_entity in comment_entities {
        let comment = translate_comment(stores, &comment_entity, 0).await?;
        comments.push(comment);
    }

    Ok(PostModel { summary, comments })
}
