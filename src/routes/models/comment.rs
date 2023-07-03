use async_recursion::async_recursion;
use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::MySqlPool;

use crate::entities::{comment::{CommentEntity, self}, EntityError, user, content};

use super::{User, translate_user, utils};

pub const MAX_CHILD_COMMENTS: u8 = 5;
const MAX_DEPTH: usize = 5; 

#[derive(Serialize)]
pub struct Comment {
    pub id: String,
    pub author: User,
    pub created: NaiveDateTime,
    pub created_pretty: String,
    pub content: String,
    pub children: Vec<Comment>,
}

#[async_recursion]
pub async fn translate_comment(
    pool: &MySqlPool,
    comment_entity: &CommentEntity,
    depth: usize,
    ) -> Result<Comment, EntityError> {
    
    let author_entity = user::get_by_id(pool, comment_entity.author_id).await?;
    let author = translate_user(author_entity);
    
    let children_entities = comment::get_by_post_id_parent_id(pool, &comment_entity.post_id, Some(comment_entity.id), None, MAX_CHILD_COMMENTS).await?;
    let mut children: Vec<Comment> = vec![];
    
    if depth < MAX_DEPTH {
        for child_entity in children_entities {
            let child_comment = translate_comment(pool, &child_entity, depth + 1).await?;
            children.push(child_comment);
        }
    }
    
    let content = content::get_by_id(pool, comment_entity.content_id).await?;
    Ok(Comment {
        id: utils::get_readable_public_id(&comment_entity.public_id),
        author,
        created: comment_entity.created,
        created_pretty: utils::format_relative_timespan(comment_entity.created),
        content: content.body,
        children
    })
}