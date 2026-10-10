use surrealdb::RecordId;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;

use crate::auth::custom_auth::hash_password;
use crate::errors::oauth::OAuthError;
use crate::errors::oauth::OAuthResult;
use crate::models::oauth::GoogleTokenResponse;
use crate::models::oauth::GoogleUser;
use crate::models::user::CreateUser;
use crate::models::user::CreateUserIdentifier;
use crate::models::user::Identifier;
use crate::models::user::User;
use crate::models::user::UserIdentifier;
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

    let url =
        reqwest::Url::parse_with_params("https://accounts.google.com/o/oauth2/v2/auth", &params)
            .map_err(|e| OAuthError::UrlBuildError(e.to_string()))?;

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
        .query("SELECT * FROM user_identifier WHERE identifier_type = 'google' AND identifier_value = $id LIMIT 1")
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

    let raw_placeholder = format!("oauth_google_{}", generate_token());
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

    let identifier = Identifier::Google(profile.id);
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
