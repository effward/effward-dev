use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::MySqlPool;

use crate::entities::{post::PostEntity, user, EntityError};

use super::{translate_user, utils, User};

#[derive(Serialize)]
pub struct PostSummary {
    pub id: String,
    pub author: User,
    pub title: String,
    pub created: NaiveDateTime,
}

pub async fn translate_post_summary(
    pool: &MySqlPool,
    post_entity: &PostEntity,
) -> Result<PostSummary, EntityError> {
    let author_entity = user::get_by_id(pool, post_entity.author).await?;
    let author = translate_user(author_entity);

    Ok(PostSummary {
        id: utils::get_readable_public_id(&post_entity.public_id),
        author,
        title: post_entity.title.to_owned(),
        created: post_entity.created,
    })
}
