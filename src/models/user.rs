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
impl CreateUser {
    pub fn new(display_name: String, password_hash: String) -> Self {
        Self {
            display_name,
            password_hash,
        }
    }
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

impl UserOnClient {
    pub fn new(id: String, display_name: String, role: String) -> Self {
        Self {
            id,
            display_name,
            role,
        }
    }
}

#[cfg(feature = "ssr")]
impl From<User> for UpdateUser {
    fn from(user: User) -> Self {
        UpdateUser::new(
            Some(user.display_name),
            Some(user.role),
            user.updated_at,
        )
    }
}

#[cfg(feature = "ssr")]
impl From<User> for UserOnClient {
    fn from(user: User) -> Self {
        UserOnClient::new(
            user.id.to_string(),
            user.display_name,
            user.role,
        )
    }
}

#[cfg(feature = "ssr")]
impl From<&User> for UpdateUser {
    fn from(user: &User) -> Self {
        UpdateUser::new(
            Some(user.display_name.clone()),
            Some(user.role.clone()),
            user.updated_at.clone(),
        )
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
impl UpdateUser {
    pub fn new(
        display_name: Option<String>,
        role: Option<String>,
        updated_at: Datetime,
    ) -> Self {
        Self {
            display_name,
            role,
            updated_at,
        }
    }
}

#[cfg(feature = "ssr")]
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserPassword {
    pub password_hash: String,
}

#[cfg(feature = "ssr")]
impl UpdateUserPassword {
    pub fn new(password_hash: String) -> Self {
        Self { password_hash }
    }
}

#[cfg(feature = "ssr")]
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserIdentifier {
    pub user: RecordId,
    pub identifier_type: String,
    pub identifier_value: String,
    pub verified: bool,
}

#[cfg(feature = "ssr")]
impl CreateUserIdentifier {
    pub fn new(identifier: Identifier, user: RecordId) -> Self {
        let normalized = identifier.normalized();
        let (identifier_type, identifier_value) = match &normalized {
            Identifier::Email(v) => ("email".to_string(), normalize_email(v)),
            Identifier::Mobile(v) => ("mobile".to_string(), normalize_mobile(v)),
            Identifier::Google(v) => ("google".to_string(), v.trim().to_string()),
            Identifier::Discord(v) => ("discord".to_string(), v.trim().to_string()),
            Identifier::Microsoft(v) => ("microsoft".to_string(), v.trim().to_string()),
        };
        Self {
            user,
            identifier_type,
            identifier_value,
            verified: false,
        }
    }
}

#[derive(Debug, Validate, Deserialize, Serialize, Clone)]
#[serde(tag = "identifier_type", content = "identifier_value")]
pub enum Identifier {
    #[serde(rename = "email")]
    Email(#[garde(email)] String),
    #[serde(rename = "mobile")]
    Mobile(
        #[garde(pattern(r"^[+]?[(]?[0-9]{1,4}[)]?[- .]?[(]?[0-9]{1,4}[)]?[- .]?[0-9]{4,10}$"))]
        String,
    ),
    #[serde(rename = "google")]
    Google(#[garde(skip)] String),
    #[serde(rename = "discord")]
    Discord(#[garde(skip)] String),
    #[serde(rename = "microsoft")]
    Microsoft(#[garde(skip)] String),
}

impl Identifier {
    pub fn identifier_type_str(&self) -> &'static str {
        match self {
            Identifier::Email(_) => "email",
            Identifier::Mobile(_) => "mobile",
            Identifier::Google(_) => "google",
            Identifier::Discord(_) => "discord",
            Identifier::Microsoft(_) => "microsoft",
        }
    }

    pub fn identifier_value(&self) -> &str {
        match self {
            Identifier::Email(v) => v,
            Identifier::Mobile(v) => v,
            Identifier::Google(v) => v,
            Identifier::Discord(v) => v,
            Identifier::Microsoft(v) => v,
        }
    }

    pub fn normalized(&self) -> Self {
        match self {
            Identifier::Email(v) => Identifier::Email(normalize_email(v)),
            Identifier::Mobile(v) => Identifier::Mobile(normalize_mobile(v)),
            Identifier::Google(v) => Identifier::Google(v.trim().to_string()),
            Identifier::Discord(v) => Identifier::Discord(v.trim().to_string()),
            Identifier::Microsoft(v) => Identifier::Microsoft(v.trim().to_string()),
        }
    }

    pub fn is_password_login_allowed(&self) -> bool {
        matches!(self, Identifier::Email(_) | Identifier::Mobile(_))
    }
}

pub fn normalize_email(raw: &str) -> String {
    raw.trim().to_lowercase()
}

pub fn normalize_mobile(raw: &str) -> String {
    let trimmed = raw.trim();
    let mut out = String::new();
    for (i, c) in trimmed.chars().filter(|c| c.is_ascii_digit() || *c == '+').enumerate() {
        if c == '+' && i != 0 {
            continue;
        }
        if c == '+' && !out.is_empty() {
            continue;
        }
        out.push(c);
    }
    out
}

#[cfg(feature = "ssr")]
#[derive(Debug, Deserialize)]
pub struct UserIdentifier {
    pub identifier_type: String,
    pub identifier_value: String,
    pub user: RecordId,
    pub created_at: Datetime,
    pub updated_at: Datetime,
    pub verified: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct UserIdentifierOnClient {
    pub identifier_type: String,
    pub identifier_value: String,
}

impl UserIdentifierOnClient {
    pub fn new(identifier_type: String, identifier_value: String) -> Self {
        UserIdentifierOnClient {
            identifier_type,
            identifier_value,
        }
    }
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
