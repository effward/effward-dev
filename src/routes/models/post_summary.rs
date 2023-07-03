use std::cmp;
use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::MySqlPool;
use substring::Substring;

use crate::entities::{post::PostEntity, user, EntityError, content, comment};

use super::{translate_user, utils, User};

const POST_PREVIEW_LENGTH: usize = 250;

#[derive(Serialize)]
pub struct PostSummary {
    pub id: String,
    pub author: User,
    pub title: String,
    pub created: NaiveDateTime,
    pub created_pretty: String,
    pub link: Option<String>,
    pub content: Option<String>,
    pub post_preview: Option<String>,
    pub comment_count: i64,
}

pub async fn translate_post_summary(
    pool: &MySqlPool,
    post_entity: &PostEntity,
    ) -> Result<PostSummary, EntityError> {
    let author_entity = user::get_by_id(pool, post_entity.author_id).await?;
    let author = translate_user(author_entity);

    let mut post_preview: Option<String> = None;
    let content = match post_entity.content_id {
        Some(id) => {
            let content = content::get_by_id(pool, id).await?;
            let preview_length = cmp::min(content.body.len(), POST_PREVIEW_LENGTH);
            let mut preview = content.body.substring(0, preview_length).to_owned();

            if content.body.len() > POST_PREVIEW_LENGTH {
                preview.push_str("…");
            }
            post_preview = Some(preview);
            Some(content.body)
        },
        None => None,
    };



    let comment_count = comment::get_count_by_post_id(pool, &post_entity.id).await?;

    Ok(PostSummary {
        id: utils::get_readable_public_id(&post_entity.public_id),
        author,
        title: post_entity.title.to_owned(),
        created: post_entity.created,
        created_pretty: utils::format_relative_timespan(post_entity.created),
        link: post_entity.link.to_owned(),
        content,
        post_preview,
        comment_count,
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
