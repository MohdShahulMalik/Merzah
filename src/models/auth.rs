use garde::Validate;
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use crate::errors::auth::AuthError;
#[cfg(feature = "ssr")]
use crate::models::user::Identifier;
#[cfg(feature = "ssr")]
use crate::models::user::{normalize_email, normalize_mobile};
#[cfg(feature = "ssr")]
use anyhow::{Result, anyhow};
#[cfg(feature = "ssr")]
use surrealdb::Surreal;
#[cfg(feature = "ssr")]
use surrealdb::engine::remote::ws::Client;

#[cfg(not(feature = "ssr"))]
use crate::models::user::Identifier;

pub const PASSWORD_MIN_LEN: usize = 8;
pub const PASSWORD_MAX_LEN: usize = 128;

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq)]
pub enum Platform {
    #[serde(rename = "web")]
    Web,
    #[serde(rename = "mobile")]
    Mobile,
}

#[derive(Debug, Validate, Deserialize, Serialize, Clone)]
pub struct RegistrationFormData {
    #[garde(length(min = 2, max = 100))]
    pub name: String,
    #[garde(dive)]
    pub identifier: Identifier,
    #[garde(length(min = 8, max = 128))]
    pub password: String,
    #[garde(skip)]
    pub platform: Platform,
}

#[derive(Debug, Validate, Deserialize, Serialize, Clone)]
pub struct LoginFormData {
    #[garde(dive)]
    pub identifier: Identifier,
    #[garde(length(min = 8, max = 128))]
    pub password: String,
    #[garde(skip)]
    pub platform: Platform,
}

impl RegistrationFormData {
    pub fn new(name: String, identifier: Identifier, password: String, platform: Platform) -> Self {
        let identifier = identifier.normalized();
        RegistrationFormData {
            name: name.trim().to_string(),
            identifier,
            password,
            platform,
        }
    }
}

impl LoginFormData {
    pub fn new(identifier: Identifier, password: String, platform: Platform) -> Self {
        let identifier = identifier.normalized();
        LoginFormData {
            identifier,
            password,
            platform,
        }
    }
}

#[cfg(feature = "ssr")]
impl RegistrationFormData {
    pub async fn validate_uniqueness(&self, db: &Surreal<Client>) -> Result<()> {
        let normalized = self.identifier.normalized();
        let identifier_type = normalized.identifier_type_str();
        let identifier_value = match &normalized {
            Identifier::Email(email) => normalize_email(email),
            Identifier::Mobile(mobile) => normalize_mobile(mobile),
            Identifier::Google(_)
            | Identifier::Discord(_)
            | Identifier::Microsoft(_) => {
                return Err(anyhow!("OAuth identifiers cannot be manually registered"));
            }
        };

        let mut result = db
            .query("SELECT * FROM user_identifier WHERE identifier_type = $type AND identifier_value = $value")
            .bind(("type", identifier_type))
            .bind(("value", identifier_value))
            .await
            .map_err(|e| AuthError::DatabaseError(Box::new(e)))?;

        let res: Vec<serde_json::Value> = result
            .take(0)
            .map_err(|_| anyhow!("Failed to parse query result"))?;

        if !res.is_empty() {
            Err(AuthError::NotUniqueError(identifier_type.to_string()))?
        } else {
            Ok(())
        }
    }
}
