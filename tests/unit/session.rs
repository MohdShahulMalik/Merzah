use crate::common::get_test_db;
use merzah::auth::custom_auth::register_user;
use merzah::auth::session::{create_session, delete_session, get_user_by_session};
use merzah::models::{auth::RegistrationFormData, user::Identifier};
use merzah::models::auth::Platform;

#[tokio::test]
async fn test_delete_session_success() -> anyhow::Result<()> {
    let db = get_test_db().await;

    // 1. Register User
    let name = "Session Test User".to_string();
    let identifier = Identifier::Email("session_test@example.com".to_string());
    let password = "password123".to_string();
    let form = RegistrationFormData::new(name, identifier, password, Platform::Web);
    let user_id = register_user(form, &db).await?;

    // 2. Create Session
    let token = create_session(user_id.clone(), &db).await?;

    // Verify session exists
    let user_from_session = get_user_by_session(&token, &db).await?;
    assert_eq!(user_from_session.id, user_id);

    // 3. Delete Session
    delete_session(&token, &db).await?;

    // 4. Verify Session is gone
    let result = get_user_by_session(&token, &db).await;
    assert!(result.is_err()); // Should be SessionNotFound

    Ok(())
}

#[tokio::test]
async fn test_delete_session_invalid_token_format() -> anyhow::Result<()> {
    let db = get_test_db().await;
    let invalid_token = "short"; // Too short, fails validation

    let result = delete_session(invalid_token, &db).await;
    assert!(result.is_err());
    Ok(())
}

#[tokio::test]
async fn test_delete_session_non_existent_token() -> anyhow::Result<()> {
    let db = get_test_db().await;
    // A valid formatted token (40-50 chars, alphanumeric/dash/underscore) that doesn't exist
    let fake_token = "a".repeat(45); 
    
    // This should succeed (idempotent delete) because the query just deletes nothing
    // BUT validate_session_token passes.
    let result = delete_session(&fake_token, &db).await;
    assert!(result.is_ok());
    
    Ok(())
}
