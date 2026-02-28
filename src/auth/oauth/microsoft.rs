use serde::Deserialize;

use crate::auth::oauth::provider::{OAuthProvider, ProviderUser};
use crate::errors::oauth::{OAuthError, OAuthResult};

#[derive(Debug, Deserialize)]
struct MicrosoftUser {
    sub: String,
    name: Option<String>,
    given_name: Option<String>,
    family_name: Option<String>,
    email: Option<String>,
    picture: Option<String>,
}

pub struct MicrosoftProvider {
    tenant_id: String,
}

impl MicrosoftProvider {
    pub fn new() -> Self {
        let tenant_id = std::env::var("MICROSOFT_TENANT_ID")
            .unwrap_or_else(|_| "common".to_string());
        Self { tenant_id }
    }
}

impl Default for MicrosoftProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl OAuthProvider for MicrosoftProvider {
    fn provider_name(&self) -> &str {
        "microsoft"
    }

    fn identifier_type(&self) -> &str {
        "microsoft"
    }

    fn client_id(&self) -> OAuthResult<String> {
        std::env::var("MICROSOFT_CLIENT_ID")
            .map_err(|_| OAuthError::MissingEnvVar("MICROSOFT_CLIENT_ID".to_string()))
    }

    fn client_secret(&self) -> OAuthResult<String> {
        std::env::var("MICROSOFT_CLIENT_SECRET")
            .map_err(|_| OAuthError::MissingEnvVar("MICROSOFT_CLIENT_SECRET".to_string()))
    }

    fn redirect_uri(&self) -> OAuthResult<String> {
        std::env::var("MICROSOFT_REDIRECT_URI")
            .map_err(|_| OAuthError::MissingEnvVar("MICROSOFT_REDIRECT_URI".to_string()))
    }

    fn tenant_id(&self) -> OAuthResult<String> {
        Ok(self.tenant_id.clone())
    }

    fn authorization_endpoint(&self) -> String {
        "https://login.microsoftonline.com/common/oauth2/v2.0/authorize".to_string()
    }

    fn token_endpoint(&self) -> String {
        "https://login.microsoftonline.com/common/oauth2/v2.0/token".to_string()
    }

    fn userinfo_endpoint(&self) -> String {
        "https://graph.microsoft.com/oidc/userinfo".to_string()
    }

    fn scopes(&self) -> String {
        "openid profile email".to_string()
    }

    async fn get_user_info(&self, access_token: &str) -> OAuthResult<ProviderUser> {
        let client = reqwest::Client::new();

        let response = client
            .get("https://graph.microsoft.com/oidc/userinfo")
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(OAuthError::InvalidResponse);
        }

        let microsoft_user: MicrosoftUser = response
            .json()
            .await
            .map_err(|e| OAuthError::ParseError(e.to_string()))?;

        let email = microsoft_user
            .email
            .ok_or(OAuthError::InvalidResponse)?;

        let name = microsoft_user.name.or_else(|| {
            match (microsoft_user.given_name, microsoft_user.family_name) {
                (Some(given), Some(family)) => Some(format!("{} {}", given, family)),
                (Some(given), None) => Some(given),
                (None, Some(family)) => Some(family),
                (None, None) => None,
            }
        });

        Ok(ProviderUser {
            id: microsoft_user.sub,
            email,
            name,
            picture: microsoft_user.picture,
        })
    }
}
