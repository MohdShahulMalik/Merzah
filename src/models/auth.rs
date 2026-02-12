use crate::models::user::Identifier;
use garde::Validate;
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use crate::errors::auth::AuthError;
#[cfg(feature = "ssr")]
use anyhow::{Result, anyhow};
#[cfg(feature = "ssr")]
use surrealdb::Surreal;
#[cfg(feature = "ssr")]
use surrealdb::engine::remote::ws::Client;

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
    #[garde(length(min = 8))]
    pub password: String,
    #[garde(skip)]
    pub platform: Platform,
}



#[derive(Debug, Validate, Deserialize, Serialize, Clone)]
pub struct LoginFormData {
    #[garde(dive)]
    pub identifier: Identifier,
    #[garde(length(min = 8))]
    pub password: String,
    #[garde(skip)]
    pub platform: Platform,
}

#[cfg(feature = "ssr")]
impl RegistrationFormData {

    pub fn new(name: String, identifier: Identifier, password: String, platform: Platform) -> Self {
        RegistrationFormData { name, identifier, password, platform }
    }

    pub async fn validate_uniqueness(&self, db: &Surreal<Client>) -> Result<()> {
        let (identifier_type, identifier_value) = match &self.identifier {
            Identifier::Email(email) => ("email", email.to_string()),
            Identifier::Mobile(mobile) => ("mobile", mobile.to_string()),
            Identifier::Workos(_) => return Err(anyhow!("WorkOS identifiers cannot be manually registered")),
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
