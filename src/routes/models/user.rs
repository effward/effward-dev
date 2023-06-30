use serde::{Deserialize, Serialize};

use crate::entities::user::UserEntity;

use super::utils;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub created: String,
}

pub fn translate_user(user_entity: UserEntity) -> User {
    User {
        id: utils::get_readable_public_id(&user_entity.public_id),
        name: user_entity.name.to_owned(),
        created: user_entity.created.to_string(),
    }
}
