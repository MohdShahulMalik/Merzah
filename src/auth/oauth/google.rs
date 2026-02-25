use surrealdb::{RecordId, Surreal};
use surrealdb::engine::remote::ws::Client;

use crate::errors::oauth::{OAuthError, OAuthResult};
use crate::models::oauth::{GoogleTokenResponse, GoogleUser};
use crate::models::user::{CreateUser, User, UserIdentifier};
use crate::utils::token_generator::generate_token;

pub fn get_authorization_url(state: &str) -> OAuthResult<String> {
    let client_id = std::env::var("GOOGLE_CLIENT_ID")
        .map_err(|_| OAuthError::MissingEnvVar("GOOGLE_CLIENT_ID".to_string()))?;
    let redirect_uri = std::env::var("GOOGLE_REDIRECT_URI")
        .map_err(|_| OAuthError::MissingEnvVar("GOOGLE_REDIRECT_URI".to_string()))?;
    
    let params = [
        ("client_id", client_id),
        ("redirect_uri", redirect_uri),
        ("response_type", "code".to_string()),
        ("scope", "openid email profile".to_string()),
        ("state", state.to_string()),
    ];
    
    let url = reqwest::Url::parse_with_params(
        "https://accounts.google.com/o/oauth2/v2/auth",
        &params,
    ).map_err(|e| OAuthError::UrlBuildError(e.to_string()))?;
    
    Ok(url.to_string())
}

pub async fn exchange_code(code: &str) -> OAuthResult<GoogleTokenResponse> {
    let client_id = std::env::var("GOOGLE_CLIENT_ID")
        .map_err(|_| OAuthError::MissingEnvVar("GOOGLE_CLIENT_ID".to_string()))?;
    let client_secret = std::env::var("GOOGLE_CLIENT_SECRET")
        .map_err(|_| OAuthError::MissingEnvVar("GOOGLE_CLIENT_SECRET".to_string()))?;
    let redirect_uri = std::env::var("GOOGLE_REDIRECT_URI")
        .map_err(|_| OAuthError::MissingEnvVar("GOOGLE_REDIRECT_URI".to_string()))?;
    
    let client = reqwest::Client::new();
    
    let response = client
        .post("https://oauth2.googleapis.com/token")
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

pub async fn get_user_info(access_token: &str) -> OAuthResult<GoogleUser> {
    let client = reqwest::Client::new();
    
    let response = client
        .get("https://www.googleapis.com/oauth2/v2/userinfo")
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await?;
    
    if !response.status().is_success() {
        return Err(OAuthError::InvalidResponse);
    }
    
    let user = response
        .json()
        .await
        .map_err(|e| OAuthError::ParseError(e.to_string()))?;
    
    Ok(user)
}

pub async fn find_or_create_user(
    profile: GoogleUser,
    db: &Surreal<Client>,
) -> OAuthResult<RecordId> {
    let existing: Option<UserIdentifier> = db
        .query("SELECT user FROM user_identifier WHERE identifier_type = 'google' AND identifier_value = $id")
        .bind(("id", profile.id.clone()))
        .await?
        .take(0)?;

    if let Some(record) = existing {
        return Ok(record.user);
    }

    let display_name = profile.name.unwrap_or_else(|| {
        profile.email
            .split('@')
            .next()
            .unwrap_or("User")
            .to_string()
    });

    let placeholder_password = format!("oauth_google_{}", generate_token());

    let user = CreateUser {
        display_name,
        password_hash: placeholder_password,
    };

    let surql = r#"
        BEGIN TRANSACTION;

        LET $created_user = (CREATE ONLY users CONTENT $user_data);

        CREATE user_identifier CONTENT {
            user: $created_user.id,
            identifier_type: 'google',
            identifier_value: $provider_id
        };

        RETURN $created_user;
        COMMIT TRANSACTION;
    "#;

    let mut result = db
        .query(surql)
        .bind(("user_data", user))
        .bind(("provider_id", profile.id))
        .await
        .map_err(|e| OAuthError::DatabaseError(Box::new(e)))?;

    let created_user: Option<User> = result.take(0)
        .map_err(|e| OAuthError::DatabaseError(Box::new(e)))?;
    let user_id = created_user
        .ok_or(OAuthError::UserNotFound)?
        .id;

    Ok(user_id)
}
