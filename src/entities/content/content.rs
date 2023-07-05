use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Content {
    pub id: u64,
    pub body: String,
    pub body_hash: Vec<u8>,
    pub created: DateTime<Utc>,
}
