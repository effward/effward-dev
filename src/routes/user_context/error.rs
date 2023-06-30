use actix_session::SessionGetError;
use shortguid::ParseError;

use crate::entities::EntityError;

#[derive(thiserror::Error, Debug, Clone)]
pub enum UserContextError {
    #[error("error with actix_session store")]
    SessionStore(String),
    #[error("error parsing user's uuid")]
    UuidParsing(String),
    #[error("entity error")]
    EntityError(EntityError),
    #[error("no authenticated user session")]
    NotAuthenticated,
}

impl std::convert::From<SessionGetError> for UserContextError {
    fn from(err: SessionGetError) -> Self {
        UserContextError::SessionStore(err.to_string())
    }
}

impl std::convert::From<ParseError> for UserContextError {
    fn from(err: ParseError) -> Self {
        UserContextError::UuidParsing(err.to_string())
    }
}

impl std::convert::From<EntityError> for UserContextError {
    fn from(err: EntityError) -> Self {
        UserContextError::EntityError(err)
    }
}
