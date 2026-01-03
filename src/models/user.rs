use garde::Validate;
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use surrealdb::{Datetime, RecordId};

#[cfg(feature = "ssr")]
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUser {
    pub display_name: String,
    pub password_hash: String,
}

#[cfg(feature = "ssr")]
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: RecordId,
    pub created_at: Datetime,
    pub display_name: String,
    pub password_hash: String,
    pub role: String,
    pub updated_at: Datetime,
}

#[cfg(feature = "ssr")]
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUser {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    pub updated_at: Datetime,
}

#[cfg(feature = "ssr")]
#[derive(Debug, Serialize)]
pub struct CreateUserIdentifier {
    #[serde(flatten)]
    pub identifier: Identifier,
    pub user: RecordId,
}

#[derive(Debug, Validate, Deserialize, Serialize, Clone)]
#[serde(tag = "identifier_type", content = "identifier_value")]
pub enum Identifier {
    #[serde(rename = "email")]
    Email(#[garde(email)] String),
    #[serde(rename = "mobile")]
    Mobile(#[garde(pattern(r"^[+]?[(]?[0-9]{1,4}[)]?[- .]?[(]?[0-9]{1,4}[)]?[- .]?[0-9]{4,10}$"))] String),
}

#[cfg(feature="ssr")]
#[derive(Debug, Deserialize)]
pub struct UserIdentifier {
    pub identifier_type: String,
    pub identifier_value: String,
    pub user: RecordId,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}

#[cfg(feature = "ssr")]
#[derive(Debug, Deserialize)]
pub struct UserIdentifierWithUser {
    pub identifier_type: String,
    pub identifier_value: String,
    pub created_at: Datetime,
    pub updated_at: Datetime,
    pub user: User,
}
