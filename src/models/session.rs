use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use surrealdb::RecordId;
#[cfg(feature = "ssr")]
use surrealdb::sql::Datetime;

#[cfg(feature = "ssr")]
pub const SESSION_DURATION_HOURS: i64 = 1;
#[cfg(feature = "ssr")]
pub const MAX_SESSIONS_PER_USER: usize = 10;

#[cfg(feature = "ssr")]
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSession {
    pub user: RecordId,
    pub session_token: String,
    pub expires_at: Datetime,
}

#[cfg(feature = "ssr")]
impl CreateSession {
    pub fn new(user: RecordId, session_token: String, expires_at: Datetime) -> Self {
        Self {
            user,
            session_token,
            expires_at,
        }
    }
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
    pub user: crate::models::user::User,
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

#[cfg(feature = "ssr")]
impl UpdateSession {
    pub fn new(session_token: Option<String>, expires_at: Option<Datetime>) -> Self {
        Self {
            session_token,
            expires_at,
        }
    }
}
