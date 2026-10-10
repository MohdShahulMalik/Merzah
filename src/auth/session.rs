use actix_web::http::header::HeaderValue;
use actix_web::http::header::SET_COOKIE;
use anyhow::Context;
use anyhow::Result;
use chrono::Duration;
use chrono::Utc;
use leptos::prelude::expect_context;
use leptos_actix::ResponseOptions;
use surrealdb::RecordId;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;
use surrealdb::sql::Datetime;

use crate::errors::session::SessionError;
use crate::models::session::CreateSession;
use crate::models::session::MAX_SESSIONS_PER_USER;
use crate::models::session::SESSION_DURATION_HOURS;
use crate::models::session::Session;
use crate::models::session::UpdateSession;
use crate::models::session_token::hash_session_token;
use crate::models::user::User;
use crate::utils::token_generator::generate_token;

pub fn session_max_age_secs() -> i64 {
    SESSION_DURATION_HOURS * 60 * 60
}

pub fn extract_bearer_token(auth_header: &str) -> Option<String> {
    auth_header
        .strip_prefix("Bearer ")
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

pub async fn create_session(user: RecordId, db: &Surreal<Client>) -> Result<String> {
    let session_token = generate_token();
    let token_hash = hash_session_token(&session_token);
    let expires_at = Datetime::from(Utc::now() + Duration::hours(SESSION_DURATION_HOURS));

    enforce_session_limit(&user, db).await?;

    let session = CreateSession::new(user, token_hash, expires_at);

    let _: Option<CreateSession> = db
        .create("sessions")
        .content(session)
        .await
        .map_err(|e| SessionError::DatabaseError(Box::new(e)))
        .with_context(|| "Failed to create a session")?;

    Ok(session_token)
}

async fn enforce_session_limit(user: &RecordId, db: &Surreal<Client>) -> Result<()> {
    let mut result = db
        .query("SELECT * FROM sessions WHERE user = $u ORDER BY created_at ASC")
        .bind(("u", user.clone()))
        .await
        .map_err(|e| SessionError::DatabaseError(Box::new(e)))?;
    let sessions: Vec<Session> = result.take(0).unwrap_or_default();
    if sessions.len() >= MAX_SESSIONS_PER_USER {
        let excess = sessions.len() + 1 - MAX_SESSIONS_PER_USER;
        for old in sessions.iter().take(excess) {
            let _ = db
                .query("DELETE ONLY $id")
                .bind(("id", old.id.clone()))
                .await;
        }
    }
    Ok(())
}

pub async fn get_user_by_session(session_token: &str, db: &Surreal<Client>) -> Result<User> {
    validate_session_token(session_token)?;
    let token_hash = hash_session_token(session_token);

    let result_from_sessions_table: Option<crate::models::session::SessionWithUser> = db
        .query("SELECT * FROM sessions WHERE session_token = $val LIMIT 1 FETCH user")
        .bind(("val", token_hash))
        .await
        .map_err(|e| SessionError::DatabaseError(Box::new(e)))
        .with_context(|| "Failed to fetch the session details")?
        .take(0)?;

    if let Some(session) = result_from_sessions_table {
        if session.expires_at <= Datetime::from(Utc::now()) {
            let _ = db
                .query("DELETE ONLY $id")
                .bind(("id", session.id))
                .await;
            Err(SessionError::SessionExpired(session.expires_at))?;
        }

        Ok(session.user)
    } else {
        Err(SessionError::SessionNotFound)?
    }
}

pub async fn delete_session(session_token: &str, db: &Surreal<Client>) -> Result<()> {
    validate_session_token(session_token)?;
    let token_hash = hash_session_token(session_token);

    let response: Option<Session> = db
        .query("SELECT * FROM sessions WHERE session_token = $val LIMIT 1")
        .bind(("val", token_hash.clone()))
        .await
        .map_err(|e| SessionError::DatabaseError(Box::new(e)))
        .with_context(|| "Failed to fetch the session to delete")?
        .take(0)?;

    if response.is_none() {
        Err(SessionError::SessionNotFound)?
    }

    let session = response.unwrap();

    db.query("DELETE sessions WHERE session_token = $val")
        .bind(("val", token_hash))
        .await
        .map_err(|e| SessionError::DatabaseError(Box::new(e)))
        .with_context(|| "Failed to delete the session ")?;

    if session.expires_at <= Datetime::from(Utc::now()) {
        Err(SessionError::SessionExpired(session.expires_at))?
    }

    Ok(())
}

pub async fn delete_all_sessions_for_user(user_id: &RecordId, db: &Surreal<Client>) -> Result<()> {
    db.query("DELETE sessions WHERE user = $u")
        .bind(("u", user_id.clone()))
        .await
        .map_err(|e| SessionError::DatabaseError(Box::new(e)))
        .with_context(|| "Failed to delete all sessions for user")?;
    Ok(())
}

pub async fn update_session_token(user_id: RecordId, db: &Surreal<Client>) -> Result<String> {
    let new_session_token = generate_token();
    let new_hash = hash_session_token(&new_session_token);

    let updated_session = UpdateSession::new(Some(new_hash), None);

    let _: Option<Session> = db
        .update(user_id.clone())
        .merge(updated_session)
        .await
        .map_err(|e| SessionError::DatabaseError(Box::new(e)))
        .with_context(|| "Failed to update the token for a user")?;

    Ok(new_session_token)
}

pub async fn update_session_expiry(user_id: RecordId, db: &Surreal<Client>) -> Result<()> {
    let session: Option<Session> = db
        .select(user_id.clone())
        .await
        .map_err(|e| SessionError::DatabaseError(Box::new(e)))
        .with_context(|| "Failed to fetch session for it to update")?;

    let session = session.ok_or(SessionError::SessionNotFound)?;
    let old_expired_at: chrono::DateTime<Utc> = session.expires_at.into();
    let new_expired_at =
        Datetime::from(old_expired_at + Duration::hours(SESSION_DURATION_HOURS));

    let updated_session = UpdateSession::new(None, Some(new_expired_at));

    let _: Option<Session> = db
        .update(user_id)
        .merge(updated_session)
        .await
        .map_err(|e| SessionError::DatabaseError(Box::new(e)))
        .with_context(|| "Failed to fetch session record to update the expiry time")?;

    Ok(())
}

pub async fn update_session_expiry_and_token(
    user_id: RecordId,
    db: &Surreal<Client>,
) -> Result<String> {
    let session: Option<Session> = db
        .select(user_id.clone())
        .await
        .map_err(|e| SessionError::DatabaseError(Box::new(e)))
        .with_context(
            || "Failed to fetch the session to update its session token and expiry time",
        )?;

    let session = session.ok_or(SessionError::SessionNotFound)?;

    let old_expired_at: chrono::DateTime<Utc> = session.expires_at.into();
    let new_expired_at =
        Datetime::from(old_expired_at + Duration::hours(SESSION_DURATION_HOURS));
    let new_session_token = generate_token();
    let new_hash = hash_session_token(&new_session_token);

    let updated_session = UpdateSession::new(Some(new_hash), Some(new_expired_at));

    let _: Option<Session> = db
        .update(user_id)
        .merge(updated_session)
        .await
        .map_err(|e| SessionError::DatabaseError(Box::new(e)))
        .with_context(|| "Failed to update session's token and expiry time")?;

    Ok(new_session_token)
}

pub async fn cleanup_expired_sessions(db: &Surreal<Client>) -> Result<()> {
    db.query("DELETE sessions WHERE expires_at <= time::now()")
        .await
        .map_err(|e| SessionError::DatabaseError(Box::new(e)))
        .with_context(|| "Failed to deleted expired sessions")?;

    Ok(())
}

pub fn set_session_cookie(session_token: &str) -> Result<()> {
    let response = expect_context::<ResponseOptions>();

    let cookie = format!(
        "__Host-session={}; Path=/; Secure; HttpOnly; SameSite=Lax; Max-Age={}",
        session_token,
        session_max_age_secs()
    );

    response.insert_header(
        SET_COOKIE,
        HeaderValue::from_str(&cookie).with_context(|| "Failed to set sesion headers")?,
    );

    Ok(())
}

pub fn remove_session_cookie() -> Result<()> {
    let response = expect_context::<ResponseOptions>();

    let cookie = "__Host-session=; Path=/; Secure; HttpOnly; SameSite=Lax; Max-Age=0";

    response.insert_header(
        SET_COOKIE,
        HeaderValue::from_str(cookie)
            .with_context(|| "Failed to set cookies for session removal")?,
    );

    Ok(())
}

pub fn validate_session_token(token: &str) -> Result<(), SessionError> {
    if token.is_empty() {
        Err(SessionError::InvalidToken)?
    }

    if token.len() < 40 || token.len() > 50 {
        Err(SessionError::InvalidToken)?
    }

    if !token
        .chars()
        .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
    {
        Err(SessionError::InvalidToken)?
    }

    Ok(())
}
