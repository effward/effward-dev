use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Comment {
    pub id: u64,
    pub public_id: String,
    pub author_id: u64,
    pub post_id: u64,
    pub parent_id: Option<u64>,
    pub content_id: u64,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}
