use cached::TimedSizedCache;
use serde::Serialize;
use sqlx::MySqlPool;

use crate::entities::{comment::{self, CommentEntity}, post::PostEntity, EntityError};

use super::{
    comment::{translate_comment, MAX_CHILD_COMMENTS},
    translate_post_summary, Comment, PostSummary,
};

#[derive(Serialize)]
pub struct Post {
    pub summary: PostSummary,
    pub comments: Vec<Comment>,
}

pub async fn translate_post(
    pool: &MySqlPool,
    post_entity: &PostEntity,
    cache: &mut TimedSizedCache<String, Vec<CommentEntity>>,
) -> Result<Post, EntityError> {
    let summary = translate_post_summary(pool, post_entity).await?;

    let comment_entities =
        comment::get_by_post_id_parent_id(pool, cache, &post_entity.id, None, None, MAX_CHILD_COMMENTS)
            .await?;

    let mut comments: Vec<Comment> = vec![];
    for comment_entity in comment_entities {
        let comment = translate_comment(pool, cache, &comment_entity, 0).await?;
        comments.push(comment);
    }

    Ok(Post { summary, comments })
}
