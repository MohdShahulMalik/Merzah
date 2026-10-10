use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct PasswordChangeRequest {
    pub current_password: String,
    pub new_password: String,
}

impl PasswordChangeRequest {
    pub fn new(current_password: String, new_password: String) -> Self {
        Self {
            current_password,
            new_password,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct PasswordResetRequest {
    pub identifier_type: String,
    pub identifier_value: String,
}

impl PasswordResetRequest {
    pub fn new(identifier_type: String, identifier_value: String) -> Self {
        Self {
            identifier_type,
            identifier_value,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct PasswordResetConfirm {
    pub reset_token: String,
    pub new_password: String,
}

impl PasswordResetConfirm {
    pub fn new(reset_token: String, new_password: String) -> Self {
        Self {
            reset_token,
            new_password,
        }
    }
}

#[cfg(feature = "ssr")]
#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePasswordResetToken {
    pub user: surrealdb::RecordId,
    pub token_hash: String,
    pub expires_at: surrealdb::sql::Datetime,
}

#[cfg(feature = "ssr")]
impl CreatePasswordResetToken {
    pub fn new(
        user: surrealdb::RecordId,
        token_hash: String,
        expires_at: surrealdb::sql::Datetime,
    ) -> Self {
        Self {
            user,
            token_hash,
            expires_at,
        }
    }
}
