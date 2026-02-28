use serde::Deserialize;

use crate::auth::oauth::provider::{OAuthProvider, ProviderUser};
use crate::errors::oauth::{OAuthError, OAuthResult};

#[derive(Debug, Deserialize)]
struct DiscordUser {
    id: String,
    username: String,
    email: String,
    avatar: Option<String>,
}

pub struct DiscordProvider;

impl DiscordProvider {
    pub fn new() -> Self {
        Self
    }
}

impl Default for DiscordProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl OAuthProvider for DiscordProvider {
    fn provider_name(&self) -> &str {
        "discord"
    }

    fn identifier_type(&self) -> &str {
        "discord"
    }

    fn client_id(&self) -> OAuthResult<String> {
        std::env::var("DISCORD_CLIENT_ID")
            .map_err(|_| OAuthError::MissingEnvVar("DISCORD_CLIENT_ID".to_string()))
    }

    fn client_secret(&self) -> OAuthResult<String> {
        std::env::var("DISCORD_CLIENT_SECRET")
            .map_err(|_| OAuthError::MissingEnvVar("DISCORD_CLIENT_SECRET".to_string()))
    }

    fn redirect_uri(&self) -> OAuthResult<String> {
        std::env::var("DISCORD_REDIRECT_URI")
            .map_err(|_| OAuthError::MissingEnvVar("DISCORD_REDIRECT_URI".to_string()))
    }

    fn tenant_id(&self) -> OAuthResult<String> {
        Ok(String::new())
    }

    fn authorization_endpoint(&self) -> String {
        "https://discord.com/oauth2/authorize".to_string()
    }

    fn token_endpoint(&self) -> String {
        "https://discord.com/api/oauth2/token".to_string()
    }

    fn userinfo_endpoint(&self) -> String {
        "https://discord.com/api/users/@me".to_string()
    }

    fn scopes(&self) -> String {
        "identify email".to_string()
    }

    async fn get_user_info(&self, access_token: &str) -> OAuthResult<ProviderUser> {
        let client = reqwest::Client::new();

        let response = client
            .get("https://discord.com/api/users/@me")
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(OAuthError::InvalidResponse);
        }

        let discord_user: DiscordUser = response
            .json()
            .await
            .map_err(|e| OAuthError::ParseError(e.to_string()))?;

        let picture = discord_user.avatar.map(|avatar| {
            format!("https://cdn.discordapp.com/avatars/{}/{}.png", discord_user.id, avatar)
        });

        Ok(ProviderUser {
            id: discord_user.id,
            email: discord_user.email,
            name: Some(discord_user.username),
            picture,
        })
    }
}
