use crate::models::user::User;
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use surrealdb::RecordId;
#[cfg(feature = "ssr")]
use surrealdb::sql::Datetime;

#[cfg(feature = "ssr")]
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSession {
    pub user: RecordId,
    pub session_token: String,
    pub expires_at: Datetime,
}

#[cfg(feature = "ssr")]
#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    pub id: RecordId,
    pub user: RecordId,
    pub session_token: String,
    pub expires_at: Datetime,
    pub created_at: Datetime,
}

#[cfg(feature = "ssr")]
#[derive(Debug, Serialize, Deserialize)]
pub struct SessionWithUser {
    pub id: RecordId,
    pub user: User,
    pub session_token: String,
    pub expires_at: Datetime,
    pub created_at: Datetime,
}

#[cfg(feature = "ssr")]
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateSession {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<Datetime>,
}
