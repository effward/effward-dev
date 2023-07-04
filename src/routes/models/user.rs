use serde::{Deserialize, Serialize};

use crate::entities::user::User;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserModel {
    pub id: String,
    pub name: String,
    pub created: String,
}

impl From<User> for UserModel {
    fn from(user: User) -> Self {
        Self {
            id: user.public_id,
            name: user.name,
            created: user.created.to_string(),
        }
    }
}
