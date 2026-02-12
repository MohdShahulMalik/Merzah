use crate::common::get_test_db;
use merzah::{
    models::{
        auth::{RegistrationFormData, Platform},
        user::{Identifier, User},
    },
    utils::user_elevation::elevate_user,
};
use surrealdb::{engine::remote::ws::Client, Surreal};
use merzah::auth::custom_auth::register_user;
use serde::Serialize;
use rstest::rstest;

#[derive(Serialize)]
struct Role {
    role: String,
}

async fn create_user(
    db: &Surreal<Client>,
    name: &str,
    email: &str,
    role: Option<&str>,
) -> User {
    // Generate unique email to avoid collision in parallel tests if any
    let unique_email = format!("{}_{}", uuid::Uuid::new_v4(), email);
    
    let form = RegistrationFormData::new(
        name.to_string(),
        Identifier::Email(unique_email),
        "password".to_string(),
        Platform::Web,
    );
    let user_id = register_user(form, db).await.expect("Failed to register user");

    if let Some(r) = role {
        // Manually update role for setup
        let _: Option<User> = db
            .update(user_id.clone())
            .merge(Role { role: r.to_string() })
            .await
            .expect("Failed to set role");
    }

    db.select(user_id).await.expect("User not found").unwrap()
}

#[rstest]
#[case::success("app_admin", "regular", "mosque_supervisor", true, None)]
#[case::unauthorized_requester("regular", "regular", "mosque_supervisor", false, Some("The user attempting the elevation is not authorized to elevate"))]
#[case::already_elevated("app_admin", "mosque_supervisor", "mosque_supervisor", false, Some("The user is already an mosque supervisor"))]
#[tokio::test]
async fn test_elevate_user(
    #[case] admin_role: &str,
    #[case] target_user_initial_role: &str,
    #[case] elevation_degree: &str,
    #[case] should_succeed: bool,
    #[case] expected_error_part: Option<&str>,
) {
    let db = get_test_db().await;
    let admin = create_user(&db, "Admin", "admin@test.com", Some(admin_role)).await;
    let target_user = create_user(&db, "Target", "target@test.com", Some(target_user_initial_role)).await;

    let result = elevate_user(
        admin.id.clone(),
        target_user.id.clone(),
        elevation_degree.to_string(),
        &db,
    )
    .await;

    if should_succeed {
        assert!(result.is_ok(), "Elevation should have succeeded but failed with: {:?}", result.err());
        assert_eq!(result.unwrap(), format!("Elevated the user to {}", elevation_degree));

        // Verify DB update
        let updated_user: User = db.select(target_user.id).await.unwrap().unwrap();
        assert_eq!(updated_user.role, elevation_degree);
    } else {
        assert!(result.is_err(), "Elevation should have failed");
        let err_msg = result.unwrap_err().to_string();
        if let Some(expected_part) = expected_error_part {
            assert!(
                err_msg.contains(expected_part),
                "Error message '{}' did not contain expected part '{}'",
                err_msg,
                expected_part
            );
        }
    }
}

#[tokio::test]
async fn test_elevate_user_target_not_found() {
    let db = get_test_db().await;
    let admin = create_user(&db, "Admin", "admin@test.com", Some("app_admin")).await;
    let fake_user_id = surrealdb::RecordId::from(("users", "nonexistent"));

    let result = elevate_user(
        admin.id.clone(),
        fake_user_id,
        "mosque_supervisor".to_string(),
        &db,
    )
    .await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("The user to be elevated was not found") || err.to_string().contains("TargetUserNotFound"));
}

#[tokio::test]
async fn test_elevate_user_admin_not_found() {
    let db = get_test_db().await;
    let fake_admin_id = surrealdb::RecordId::from(("users", "nonexistent_admin"));
    let target_user = create_user(&db, "Target", "target@test.com", Some("regular")).await;

    let result = elevate_user(
        fake_admin_id,
        target_user.id.clone(),
        "mosque_supervisor".to_string(),
        &db,
    )
    .await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("The admin that's elevating the user was not found") || err.to_string().contains("AdminNotFound"));
}
