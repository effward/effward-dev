use redis::RedisError;

#[derive(thiserror::Error, Debug, Clone)]
pub enum ServerError {
    #[error("Unknown error")]
    Unknown(String),
    #[error("Unknown environment error, set EFFWARD_DEV_ENVIRONMENT to prod or dev")]
    Environment,
    #[error("Database initialization error")]
    DatabaseInit(String),
    #[error("Redis initialization error")]
    RedisInit(String),
    #[error("Tera initialization error")]
    TeraInit(String),
    #[error("HttpServer initialization error")]
    HttpInit(String),
    #[error("HttpServer runtime error")]
    HttpRuntime(String),
}

impl From<sqlx::Error> for ServerError {
    fn from(err: sqlx::Error) -> Self {
        ServerError::DatabaseInit(format!("📚🔥 {}", err.to_string()))
    }
}

impl From<RedisError> for ServerError {
    fn from(err: RedisError) -> Self {
        ServerError::RedisInit(format!("🗝️🔥 {}", err.to_string()))
    }
}

impl From<tera::Error> for ServerError {
    fn from(err: tera::Error) -> Self {
        ServerError::TeraInit(format!("🌎🔥 {}", err.to_string()))
    }
}

impl From<anyhow::Error> for ServerError {
    fn from(err: anyhow::Error) -> Self {
        ServerError::Unknown(format!("👤🔥 {}", err.to_string()))
    }
}

impl From<std::io::Error> for ServerError {
    fn from(err: std::io::Error) -> Self {
        ServerError::HttpInit(format!("🚀🔥 {}", err.to_string()))
    }
}
