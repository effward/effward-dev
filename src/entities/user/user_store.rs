use async_trait::async_trait;
use secrecy::Secret;

use crate::entities::EntityError;

use super::User;

#[async_trait]
pub trait UserStore: Send + Sync {
    async fn insert(
        &mut self,
        name: &str,
        email: &str,
        password: &Secret<String>,
    ) -> Result<User, EntityError>;

    async fn get_by_name_password(
        &self,
        name: &str,
        password: &Secret<String>,
    ) -> Result<User, EntityError>;

    async fn get_by_name(&self, name: &str) -> Result<User, EntityError>;

    async fn get_by_id(&self, id: u64) -> Result<User, EntityError>;

    async fn get_by_public_id(&self, public_id: &str) -> Result<User, EntityError>;
}
