use anyhow::{Context, Result, anyhow};
use argon2::Argon2;
use argon2::password_hash::PasswordHasher;
use argon2::password_hash::PasswordVerifier;
use argon2::password_hash::SaltString;
use garde::Validate;
use rand::rngs::OsRng;
use surrealdb::RecordId;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;

use crate::auth::session::delete_all_sessions_for_user;
use crate::errors::auth::AuthError;
use crate::models::auth::LoginFormData;
use crate::models::auth::PASSWORD_MAX_LEN;
use crate::models::auth::PASSWORD_MIN_LEN;
use crate::models::auth::RegistrationFormData;
use crate::models::password_reset::CreatePasswordResetToken;
use crate::models::session_token::hash_reset_token;
use crate::models::user::CreateUser;
use crate::models::user::CreateUserIdentifier;
use crate::models::user::Identifier;
use crate::models::user::UpdateUserPassword;
use crate::models::user::User;
use crate::models::user::UserIdentifierWithUser;
use crate::models::user::normalize_email;
use crate::models::user::normalize_mobile;
use crate::utils::token_generator::generate_token;

fn is_unique_violation(err: &anyhow::Error) -> bool {
    let msg = format!("{:?}", err).to_lowercase();
    msg.contains("already exists")
        || msg.contains("duplicate")
        || msg.contains("unique")
        || msg.contains("idx_identifier_value")
        || msg.contains("idx_user_identifier_type")
}

pub fn hash_password(plain: &str) -> Result<String, AuthError> {
    validate_password_policy(plain).map_err(AuthError::WeakPassword)?;
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(plain.as_bytes(), &salt)
        .map_err(AuthError::PasswordHashError)?;
    Ok(hash.to_string())
}

pub fn validate_password_policy(plain: &str) -> Result<(), String> {
    if plain.len() < PASSWORD_MIN_LEN {
        return Err(format!("password must be at least {} characters", PASSWORD_MIN_LEN));
    }
    if plain.len() > PASSWORD_MAX_LEN {
        return Err(format!("password must be at most {} characters", PASSWORD_MAX_LEN));
    }
    Ok(())
}

fn normalized_type_and_value(identifier: &Identifier) -> (String, String) {
    let normalized = identifier.normalized();
    match &normalized {
        Identifier::Email(email) => ("email".to_string(), normalize_email(email)),
        Identifier::Mobile(mobile) => ("mobile".to_string(), normalize_mobile(mobile)),
        Identifier::Google(v) => ("google".to_string(), v.trim().to_string()),
        Identifier::Discord(v) => ("discord".to_string(), v.trim().to_string()),
        Identifier::Microsoft(v) => ("microsoft".to_string(), v.trim().to_string()),
    }
}

pub async fn register_user(form: RegistrationFormData, db: &Surreal<Client>) -> Result<RecordId> {
    form.validate()
        .map_err(AuthError::InvalidData)
        .with_context(|| "The form validation for registration failed")?;
    form.validate_uniqueness(db).await?;

    let password_hash_str = hash_password(&form.password)?;

    let normalized_identifier = form.identifier.normalized();
    if !normalized_identifier.is_password_login_allowed() {
        return Err(anyhow!("OAuth identifiers cannot be manually registered"))?;
    }

    let user = CreateUser::new(form.name, password_hash_str);

    let created_user: Option<User> = db
        .create("users")
        .content(user)
        .await
        .map_err(|e| AuthError::DatabaseError(Box::new(e)))
        .with_context(|| "Failed to create user record")?;
    let created_user = created_user.ok_or_else(|| anyhow!("User creation returned no data"))?;

    let identifier_row = CreateUserIdentifier::new(normalized_identifier.clone(), created_user.id.clone());
    let ident_result = db
        .create::<Option<CreateUserIdentifier>>("user_identifier")
        .content(identifier_row)
        .await;
    if let Err(e) = ident_result {
        let _ = db
            .query("DELETE ONLY $uid")
            .bind(("uid", created_user.id.clone()))
            .await;
        let _ctx_err = anyhow!("{:?}", e);
        if is_unique_violation(&anyhow!("{:?}", e)) {
            let (t, _) = normalized_type_and_value(&normalized_identifier);
            return Err(anyhow!(AuthError::NotUniqueError(t)))?;
        }
        return Err(anyhow!(AuthError::DatabaseError(Box::new(e))))?;
    }

    Ok(created_user.id)
}

pub async fn authenticate(form: LoginFormData, db: &Surreal<Client>) -> Result<RecordId> {
    form.validate()
        .map_err(AuthError::InvalidData)
        .with_context(|| "The form validation for login failed")?;

    if !form.identifier.is_password_login_allowed() {
        return Err(anyhow!(AuthError::UserNotFound))?;
    }
    let (identifier_type, identifier_value) = normalized_type_and_value(&form.identifier);

    let mut result = db
        .query(
            "SELECT * FROM user_identifier WHERE identifier_type = $identifier_type AND identifier_value = $identifier_value FETCH user",
        )
        .bind(("identifier_type", identifier_type))
        .bind(("identifier_value", identifier_value))
        .await
        .map_err(|e| AuthError::DatabaseError(Box::new(e)))
        .with_context(|| "Failed to get search for the identifier for authentication")?;

    let user_identifier_with_user_option: Option<UserIdentifierWithUser> = result
        .take(0)
        .map_err(|e| AuthError::DatabaseError(Box::new(e)))
        .with_context(|| "failed to get the result for the request user identifier")?;
    let user_identifier_with_user: UserIdentifierWithUser =
        user_identifier_with_user_option.ok_or(AuthError::UserNotFound)?;

    let requested_user = user_identifier_with_user.user;

    let parsed_hash = argon2::password_hash::PasswordHash::new(&requested_user.password_hash)
        .map_err(AuthError::PasswordHashError)?;

    let argon2 = Argon2::default();
    argon2
        .verify_password(form.password.as_bytes(), &parsed_hash)
        .map_err(AuthError::PasswordVerificationError)
        .with_context(|| "Password verification failed")?;

    Ok(requested_user.id)
}

pub async fn change_password(
    user_id: RecordId,
    current_password: &str,
    new_password: &str,
    db: &Surreal<Client>,
) -> Result<()> {
    validate_password_policy(new_password).map_err(AuthError::WeakPassword)?;
    let user: Option<User> = db
        .select(user_id.clone())
        .await
        .map_err(|e| AuthError::DatabaseError(Box::new(e)))?;
    let user = user.ok_or(AuthError::UserNotFound)?;

    let parsed_hash = argon2::password_hash::PasswordHash::new(&user.password_hash)
        .map_err(AuthError::PasswordHashError)?;
    Argon2::default()
        .verify_password(current_password.as_bytes(), &parsed_hash)
        .map_err(AuthError::PasswordVerificationError)?;

    let new_hash = hash_password(new_password)?;
    let patch = UpdateUserPassword::new(new_hash);
    let _: Option<User> = db
        .update(user_id.clone())
        .merge(patch)
        .await
        .map_err(|e| AuthError::DatabaseError(Box::new(e)))?;

    delete_all_sessions_for_user(&user_id, db)
        .await
        .with_context(|| "Failed to invalidate sessions after password change")?;
    Ok(())
}

pub async fn request_password_reset(
    identifier: Identifier,
    db: &Surreal<Client>,
) -> Result<String> {
    let (identifier_type, identifier_value) = normalized_type_and_value(&identifier);
    let mut result = db
        .query(
            "SELECT * FROM user_identifier WHERE identifier_type = $t AND identifier_value = $v",
        )
        .bind(("t", identifier_type))
        .bind(("v", identifier_value))
        .await
        .map_err(|e| AuthError::DatabaseError(Box::new(e)))?;
    let found: Option<crate::models::user::UserIdentifier> = result
        .take(0)
        .map_err(|e| AuthError::DatabaseError(Box::new(e)))?;
    let found = found.ok_or(AuthError::UserNotFound)?;

    let raw_token = generate_token();
    let token_hash = hash_reset_token(&raw_token);
    let expires_at = surrealdb::sql::Datetime::from(chrono::Utc::now() + chrono::Duration::hours(1));
    let row = CreatePasswordResetToken::new(found.user, token_hash, expires_at);
    let _: Option<CreatePasswordResetToken> = db
        .create("password_reset_tokens")
        .content(row)
        .await
        .map_err(|e| AuthError::DatabaseError(Box::new(e)))?;
    Ok(raw_token)
}

pub async fn confirm_password_reset(
    raw_token: &str,
    new_password: &str,
    db: &Surreal<Client>,
) -> Result<()> {
    validate_password_policy(new_password).map_err(AuthError::WeakPassword)?;
    let token_hash = hash_reset_token(raw_token);
    let mut result = db
        .query("SELECT * FROM password_reset_tokens WHERE token_hash = $h FETCH user")
        .bind(("h", token_hash.clone()))
        .await
        .map_err(|e| AuthError::DatabaseError(Box::new(e)))?;
    #[derive(Debug, serde::Deserialize)]
    struct ResetRow {
        id: RecordId,
        user: User,
        expires_at: surrealdb::sql::Datetime,
    }
    let row: Option<ResetRow> = result
        .take(0)
        .map_err(|e| AuthError::DatabaseError(Box::new(e)))?;
    let row = row.ok_or(AuthError::InvalidResetToken)?;
    let now = surrealdb::sql::Datetime::from(chrono::Utc::now());
    if row.expires_at <= now {
        let _ = db
            .query("DELETE ONLY $id")
            .bind(("id", row.id))
            .await;
        return Err(anyhow!(AuthError::InvalidResetToken))?;
    }
    let new_hash = hash_password(new_password)?;
    let user_id = row.user.id.clone();
    let patch = UpdateUserPassword::new(new_hash);
    let _: Option<User> = db
        .update(user_id.clone())
        .merge(patch)
        .await
        .map_err(|e| AuthError::DatabaseError(Box::new(e)))?;
    let _ = db
        .query("DELETE ONLY $id")
        .bind(("id", row.id))
        .await;
    delete_all_sessions_for_user(&user_id, db).await?;
    Ok(())
}
