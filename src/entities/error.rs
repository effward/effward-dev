#[derive(thiserror::Error, Debug, Clone)]
pub enum EntityError {
    #[error("Internal error")]
    Internal(String),
    #[error("Invalid input")]
    InvalidInput(&'static str, &'static str),
    #[error("Malformed data")]
    MalformedData,
    #[error("Entity not found")]
    NotFound,
    #[error("Duplicate key")]
    DuplicateKey,
}

impl std::convert::From<sqlx::Error> for EntityError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => EntityError::NotFound,
            sqlx::Error::Database(db) => match db.code() {
                Some(code) => match code.to_string().as_str() {
                    "23000" => EntityError::DuplicateKey,
                    _ => EntityError::Internal(format!(
                        "SQLx DatabaseError. SQLSTATE Code: {}\nerror: {:?}",
                        code.to_string(),
                        db
                    )),
                },
                None => EntityError::Internal(format!("SQLx DatabaseError. error: {:?}", db)),
            },
            _ => EntityError::Internal(format!("SQLx Error: {:?}", err)),
        }
    }
}
