#[cfg(feature = "ssr")]
use thiserror::Error;

#[cfg(feature = "ssr")]
#[derive(Debug, Error)]
pub enum EducationError {
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Not found")]
    NotFound,
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Database operation failed")]
    DatabaseError(#[from] surrealdb::Error),
}
