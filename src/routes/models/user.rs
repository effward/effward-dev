use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::entities::user::UserModel;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub created: String,
}

pub fn translate_user(user: UserModel) -> User {
    let mut bytes: [u8; 16] = [0; 16];
    let mut i = 0;
    for byte in &user.public_id {
        bytes[i] = *byte;
        i += 1;

        if i >= 16 {
            break;
        }
    }
    let public_id = Uuid::from_bytes(bytes);
    return User {
        id: public_id,
        name: user.name.to_owned(),
        created: user.created.to_string(),
    };
}
