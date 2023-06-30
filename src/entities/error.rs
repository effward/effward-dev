#[derive(thiserror::Error, Debug, Clone)]
pub enum EntityError {
    #[error("internal error")]
    Internal(String),
    #[error("invalid input")]
    InvalidInput(&'static str, &'static str),
    #[error("malformed data")]
    MalformedData,
    #[error("entity not found")]
    NotFound,
    #[error("duplicate key")]
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
