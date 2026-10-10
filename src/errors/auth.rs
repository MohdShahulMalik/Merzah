#[cfg(feature = "ssr")]
use thiserror::Error;

#[cfg(feature = "ssr")]
#[derive(Debug, Error)]
pub enum AuthError {
    #[error("The form data provided is invalid")]
    InvalidData(#[from] garde::Report),

    #[error("Database operation failed")]
    DatabaseError(#[from] Box<surrealdb::Error>),

    #[error("{0} already registered")]
    NotUniqueError(String),

    #[error("Failed to hash the password")]
    PasswordHashError(argon2::password_hash::Error),

    #[error("Password verification failed")]
    PasswordVerificationError(argon2::password_hash::Error),

    #[error("Requested user was not found")]
    UserNotFound,

    #[error("Too many login attempts. Try again in {0} seconds")]
    RateLimited(u64),

    #[error("Password does not meet strength requirements: {0}")]
    WeakPassword(String),

    #[error("Password reset token is invalid or expired")]
    InvalidResetToken,
}
