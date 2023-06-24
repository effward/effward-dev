#[derive(thiserror::Error, Debug, Clone)]
pub enum EntityError {
    #[error("Internal error")]
    Internal(String),
    #[error("Invalid password")]
    InvalidPassword,
    #[error("Malformed data")]
    MalformedData,
    #[error("Entity not found")]
    NotFound,
}

impl std::convert::From<sqlx::Error> for EntityError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => EntityError::NotFound,
            sqlx::Error::Database(db) => {
                match db.code() {
                    Some(code) => {
                        println!("🔥🔥🔥 SQLSTATE code: {}", code.to_string());
                        EntityError::Internal(db.to_string())
                    }
                    None => {
                        EntityError::Internal(db.to_string())
                    }
                }
                
            }
            _ => EntityError::Internal(err.to_string())
        }
    }
}