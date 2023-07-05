use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Email {
    pub id: u64,
    pub address: String,
    pub created: DateTime<Utc>,
}
