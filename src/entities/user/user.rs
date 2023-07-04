use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub id: u64,
    pub public_id: String,
    pub name: String,
    pub email_id: u64,
    pub is_deleted: bool,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}
