use thiserror::Error;

#[derive(Debug, Error)]
pub enum OAuthError {
    #[error("Missing required environment variable: {0}")]
    MissingEnvVar(String),

    #[error("Failed to build authorization URL: {0}")]
    UrlBuildError(String),

    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("Failed to parse JSON response: {0}")]
    ParseError(String),

    #[error("User not found")]
    UserNotFound,

    #[error("Database error: {0}")]
    DatabaseError(#[from] Box<surrealdb::Error>),

    #[error("Invalid response from OAuth provider")]
    InvalidResponse,
}

impl From<surrealdb::Error> for OAuthError {
    fn from(err: surrealdb::Error) -> Self {
        OAuthError::DatabaseError(Box::new(err))
    }
}

pub type OAuthResult<T> = Result<T, OAuthError>;

#[derive(Debug, Error)]
pub enum StateError {
    #[error("Failed to generate state token")]
    GenerationError,
    #[error("State validation failed")]
    ValidationFailed,
}

pub type StateResult<T> = Result<T, StateError>;
