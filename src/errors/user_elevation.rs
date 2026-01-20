#[cfg(feature="ssr")]
use thiserror::Error;

#[cfg(feature="ssr")]
#[derive(Debug, Error)]
pub enum UserElevationError {
    #[error("Database operation failed")]
    DatabaseError(#[from] surrealdb::Error),

    #[error("The user attempting the elevation is not authorized to elevate")]
    Unauthorized,

    #[error("The user to be elevated was not found")]
    TargetUserNotFound,

    #[error("The admin that's elevating the user was not found")]
    AdminNotFound,

    #[error("The user is already an {0}")]
    AlreadyElevated(String),
    
    #[error("Cannot elevate self")]
    SelfElevationNotAllowed,
}
