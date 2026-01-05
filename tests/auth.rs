mod common;
use common::get_test_db;
use merzah::{
    models::{api_responses::ApiResponse, auth::RegistrationFormData, user::Identifier},
    spawn_app,
};
use reqwest::Client;
use rstest::rstest;
use serde::Serialize;

#[derive(Serialize)]
struct Form {
    form: RegistrationFormData,
}

#[rstest]
#[case::mobile("Armaan Ali".to_string(), Identifier::Mobile("+91 1234567890".to_string()), "thisisasecret".to_string(), Some("The user have been registered successfully".to_string()), "Payload with Identifier Type mobile")]
#[case::email("Armaan Ali".to_string(), Identifier::Email("armaanali@gmail.com".to_string()), "thisisasecret".to_string(), Some("The user have been registered successfully".to_string()), "Payload with Identifier Type email")]
#[tokio::test]
async fn register_server_fn_successfully_register_a_user(
    #[case] name: String,
    #[case] identifier: Identifier,
    #[case] password: String,
    #[case] expected_response_data: Option<String>,
    #[case] payload_info: &str,
) {
    let client = Client::new();
    let db = get_test_db().await;
    let addr = spawn_app(db.clone());
    let relative_addr = format!("{}/auth/register", addr);

    let body = Form {
        form: RegistrationFormData::new(name.clone(), identifier.clone(), password.clone()),
    };

    let response = client
        .post(relative_addr)
        .json(&body)
        .send()
        .await
        .expect("Failed to send a request");

    if !response.status().is_success() {
        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        println!(
            "Request failed for {}. Status: {}, Body: {}",
            payload_info, status, text
        );
        panic!("Expected successful response status");
    }

    let api_response = response
        .json::<ApiResponse<String>>()
        .await
        .expect("Failed to deserialize response");

    let actual_response_data = api_response.data;

    assert_eq!(actual_response_data, expected_response_data);
    assert!(api_response.error.is_none());

    // Verify DB State
    let (id_type, id_value) = match identifier {
        Identifier::Email(e) => ("email", e),
        Identifier::Mobile(m) => ("mobile", m),
    };

    // 1. Verify User Identifier exists
    let mut result = db
        .query("SELECT * FROM user_identifier WHERE identifier_type = $type AND identifier_value = $val FETCH user")
        .bind(("type", id_type))
        .bind(("val", id_value.clone()))
        .await
        .expect("Failed to query user identifier");

    let error_msg = format!(
        "User identifier record should exist, payload info: {}",
        payload_info
    );
    let user_identifier_with_user: Option<merzah::models::user::UserIdentifierWithUser> =
        result.take(0).expect("Failed to parse user identifier");
    assert!(user_identifier_with_user.is_some(), "{}", error_msg);

    let user_identifier_with_user = user_identifier_with_user.unwrap();
    assert_eq!(user_identifier_with_user.identifier_type, id_type);
    assert_eq!(user_identifier_with_user.identifier_value, id_value);

    let user: merzah::models::user::User = user_identifier_with_user.user;
    let user_id = user.id;

    // 2. Verify User exists and has correct name
    assert_eq!(user.display_name, name);

    // 3. Verify Session was created
    let mut session_result = db
        .query("SELECT * FROM sessions WHERE user = $user")
        .bind(("user", user_id))
        .await
        .expect("Failed to query sessions");

    let error_msg = format!(
        "A session should be created for the registered user, payload info: {}",
        payload_info
    );
    let sessions: Vec<merzah::models::session::Session> =
        session_result.take(0).expect("Failed to parse sessions");
    assert!(!sessions.is_empty(), "{}", error_msg);
}
