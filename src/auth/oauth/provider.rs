use serde::Deserialize;
use surrealdb::RecordId;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;

use crate::auth::custom_auth::hash_password;
use crate::errors::oauth::OAuthError;
use crate::errors::oauth::OAuthResult;
use crate::models::user::CreateUser;
use crate::models::user::CreateUserIdentifier;
use crate::models::user::Identifier;
use crate::models::user::User;
use crate::models::user::UserIdentifier;
use crate::utils::token_generator::generate_token;

#[derive(Debug, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub expires_in: i64,
    pub token_type: String,
    pub scope: String,
    pub refresh_token: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ProviderUser {
    pub id: String,
    pub email: String,
    pub name: Option<String>,
    pub picture: Option<String>,
}

impl ProviderUser {
    pub fn new(id: String, email: String, name: Option<String>, picture: Option<String>) -> Self {
        Self {
            id,
            email,
            name,
            picture,
        }
    }
}

#[allow(async_fn_in_trait)]
pub trait OAuthProvider: Send + Sync {
    fn provider_name(&self) -> &str;
    fn identifier_type(&self) -> &str;

    fn client_id(&self) -> OAuthResult<String>;
    fn client_secret(&self) -> OAuthResult<String>;
    fn redirect_uri(&self) -> OAuthResult<String>;
    fn tenant_id(&self) -> OAuthResult<String>;

    fn authorization_url(&self, state: &str) -> OAuthResult<String> {
        let client_id = self.client_id()?;
        let redirect_uri = self.redirect_uri()?;

        let params = [
            ("client_id", client_id),
            ("redirect_uri", redirect_uri),
            ("response_type", "code".to_string()),
            ("scope", self.scopes().to_string()),
            ("state", state.to_string()),
        ];

        let url = reqwest::Url::parse_with_params(&self.authorization_endpoint(), &params)
            .map_err(|e| OAuthError::UrlBuildError(e.to_string()))?;

        Ok(url.to_string())
    }

    fn authorization_endpoint(&self) -> String;
    fn token_endpoint(&self) -> String;
    fn userinfo_endpoint(&self) -> String;
    fn scopes(&self) -> String;

    async fn exchange_code(&self, code: &str) -> OAuthResult<TokenResponse> {
        let client_id = self.client_id()?;
        let client_secret = self.client_secret()?;
        let redirect_uri = self.redirect_uri()?;

        let client = reqwest::Client::new();

        let response = client
            .post(self.token_endpoint())
            .form(&[
                ("client_id", client_id.as_str()),
                ("client_secret", client_secret.as_str()),
                ("code", code),
                ("grant_type", "authorization_code"),
                ("redirect_uri", redirect_uri.as_str()),
            ])
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(OAuthError::InvalidResponse);
        }

        let token_response = response
            .json()
            .await
            .map_err(|e| OAuthError::ParseError(e.to_string()))?;

        Ok(token_response)
    }

    async fn get_user_info(&self, access_token: &str) -> OAuthResult<ProviderUser>;

    async fn find_or_create_user(
        &self,
        profile: ProviderUser,
        db: &Surreal<Client>,
    ) -> OAuthResult<RecordId> {
        let identifier_type = self.identifier_type().to_string();

        let existing: Option<UserIdentifier> = db
            .query("SELECT * FROM user_identifier WHERE identifier_type = $id_type AND identifier_value = $id LIMIT 1")
            .bind(("id_type", identifier_type.clone()))
            .bind(("id", profile.id.clone()))
            .await?
            .take(0)?;

        if let Some(record) = existing {
            return Ok(record.user);
        }

        let display_name = profile.name.unwrap_or_else(|| {
            profile
                .email
                .split('@')
                .next()
                .unwrap_or("User")
                .to_string()
        });

        let raw_placeholder = format!("oauth_{}_{}", identifier_type, generate_token());
        let placeholder_hash = hash_password(&raw_placeholder)
            .map_err(|e| OAuthError::ParseError(format!("Failed to hash OAuth placeholder: {}", e)))?;

        let user = CreateUser::new(display_name, placeholder_hash);

        let created: Option<User> = db
            .create("users")
            .content(user)
            .await
            .map_err(|e| OAuthError::DatabaseError(Box::new(e)))?;
        let created_user = created.ok_or(OAuthError::UserNotFound)?;
        let user_id = created_user.id.clone();

        let identifier = match identifier_type.as_str() {
            "google" => Identifier::Google(profile.id.clone()),
            "discord" => Identifier::Discord(profile.id.clone()),
            "microsoft" => Identifier::Microsoft(profile.id.clone()),
            _ => Identifier::Google(profile.id.clone()),
        };
        let identifier_row = CreateUserIdentifier::new(identifier, user_id.clone());
        let ident_result = db
            .create::<Option<CreateUserIdentifier>>("user_identifier")
            .content(identifier_row)
            .await;
        if let Err(e) = ident_result {
            let _ = db
                .query("DELETE ONLY $uid")
                .bind(("uid", user_id.clone()))
                .await;
            return Err(OAuthError::DatabaseError(Box::new(e)));
        }

        Ok(user_id)
    }
}
