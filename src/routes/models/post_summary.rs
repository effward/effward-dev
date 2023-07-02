use chrono::{NaiveDateTime, Utc};
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
    pub created_pretty: String,
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
        created_pretty: format_relative_timespan(post_entity.created),
    })
}

pub fn format_relative_timespan(datetime: NaiveDateTime) -> String {
    let now = Utc::now().naive_utc();
    let difference = now - datetime;

    let minutes = difference.num_minutes();
    if minutes < 60 {
        return format!("{} minutes", minutes);
    }

    let hours = difference.num_hours();
    if hours < 24 {
        return format!("{} hours", hours);
    }

    let days = difference.num_days();
    if days < 7 {
        return format!("{} days", days);
    }

    let weeks = days / 7;
    if weeks < 4 {
        return format!("{} weeks", weeks);
    }

    let months = days / 30;
    if months < 12 {
        return format!("{} months", months);
    }

    let years = months / 12;
    format!("{} years", years)
}
