use r2d2_mongodb::mongodb::Error as MongoError;
use serde::Serialize;

// Generic RepositoryError
#[derive(Debug, Serialize)]
pub enum RepositoryError {
    NotFound(String),
    Unauthorized(String),
    Default(String),
    Internal(String),
}

impl From<MongoError> for RepositoryError {
    fn from(err: MongoError) -> Self {
        RepositoryError::Internal(err.to_string())
    }
}
