use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::entities::user::User;

use super::utils;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserModel {
    pub id: String,
    pub name: String,
    pub created: DateTime<Utc>,
    pub created_pretty: String,
}

impl From<User> for UserModel {
    fn from(user: User) -> Self {
        Self {
            id: user.public_id,
            name: user.name,
            created: user.created,
            created_pretty: utils::get_readable_duration(user.created),
        }
    }
}
