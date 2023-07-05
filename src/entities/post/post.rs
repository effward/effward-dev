use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Post {
    pub id: u64,
    pub public_id: String,
    pub author_id: u64,
    pub title: String,
    pub link: Option<String>,
    pub content_id: Option<u64>,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}
