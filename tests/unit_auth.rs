mod common;
use common::get_test_db;
use merzah::auth::custom_auth::register_user;
use merzah::models::{auth::RegistrationFormData, user::Identifier};

#[tokio::test]
async fn test_register_user_success() -> anyhow::Result<()> {
    let db = get_test_db().await;
    
    let name = "Unit Test User".to_string();
    let identifier = Identifier::Email("unit_test@example.com".to_string());
    let password = "password123".to_string();
    
    let form = RegistrationFormData::new(name.clone(), identifier.clone(), password.clone());
    
    let user_id = register_user(form, &db).await?;
    
    // Verify user created
    let user: Option<merzah::models::user::User> = db.select(user_id).await?;
    assert!(user.is_some());
    assert_eq!(user.unwrap().display_name, name);
    Ok(())
}

#[tokio::test]
async fn test_register_user_duplicate_fail() -> anyhow::Result<()> {
    let db = get_test_db().await;
    
    let name = "Duplicate User".to_string();
    let identifier = Identifier::Email("duplicate@example.com".to_string());
    let password = "password123".to_string();
    
    let form1 = RegistrationFormData::new(name.clone(), identifier.clone(), password.clone());
    let form2 = RegistrationFormData::new(name.clone(), identifier.clone(), password.clone()); // Same identifier
    
    // First registration
    register_user(form1, &db).await?;
    
    // Second registration should fail due to uniqueness check or DB constraint
    let result2 = register_user(form2, &db).await;
    assert!(result2.is_err(), "Duplicate registration should fail");
    Ok(())
}
