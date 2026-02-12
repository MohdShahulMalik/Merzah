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
impl User {
    pub fn is_app_admin(&self) -> bool {
        self.role == "app_admin"
    }

    pub fn is_mosque_supervisor(&self) -> bool {
        self.role == "mosque_supervisor"
    }

    pub fn elevate_to(&mut self, elevation_degree: String) {
        self.role = elevation_degree;
        self.refresh_updated_at();
    }

    pub fn refresh_updated_at(&mut self) {
        use chrono::Utc;

        self.updated_at = Utc::now().into();
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UserOnClient {
    pub id: String,
    pub display_name: String,
    pub role: String,
}

#[cfg(feature = "ssr")]
impl From<User> for UpdateUser {
    fn from(user: User) -> Self {
        UpdateUser {
            display_name: Some(user.display_name),
            role: Some(user.role),
            updated_at: user.updated_at,
        }
    }
}

#[cfg(feature = "ssr")]
impl From<User> for UserOnClient {
    fn from(user: User) -> Self {
        UserOnClient {
            id: user.id.to_string(),
            display_name: user.display_name,
            role: user.role,
        }
    }
}

#[cfg(feature = "ssr")]
impl From<&User> for UpdateUser {
    fn from(user: &User) -> Self {
        UpdateUser {
            display_name: Some(user.display_name.clone()),
            role: Some(user.role.clone()),
            updated_at: user.updated_at.clone(),
        }
    }
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
    #[serde(rename = "workos")]
    Workos(#[garde(skip)] String),
}

#[cfg(feature = "ssr")]
#[derive(Debug, Deserialize)]
pub struct UserIdentifier {
    pub identifier_type: String,
    pub identifier_value: String,
    pub user: RecordId,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct UserIdentifierOnClient {
    pub identifier_type: String,
    pub identifier_value: String,
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
