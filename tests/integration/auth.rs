use crate::common::get_test_db;
use merzah::{
    models::{api_responses::ApiResponse, auth::{LoginFormData, RegistrationFormData, Platform}, user::Identifier},
    spawn_app,
};
use reqwest::Client;
use rstest::rstest;
use serde::Serialize;

#[derive(Serialize)]
pub struct RegisterationFormWrapper {
    pub form: RegistrationFormData,
}

#[derive(Serialize)]
struct LoginFormWrapper {
    form: LoginFormData,
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

    let body = RegisterationFormWrapper {
        form: RegistrationFormData::new(name.clone(), identifier.clone(), password.clone(), Platform::Web),
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
        Identifier::Workos(_) => todo!(),
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

#[tokio::test]
async fn logout_server_fn_successfully_logs_out_user() {
    let client = Client::new();
    let db = get_test_db().await;
    let addr = spawn_app(db.clone());
    let register_url = format!("{}/auth/register", addr);
    let logout_url = format!("{}/auth/logout", addr);

    let form = RegistrationFormData::new(
        "Logout User".to_string(),
        Identifier::Email("logout@example.com".to_string()),
        "password123".to_string(),
        Platform::Web,
    );
    let body = RegisterationFormWrapper { form };

    // 1. Register
    let response = client
        .post(&register_url)
        .json(&body)
        .send()
        .await
        .expect("Failed to register");

    assert!(response.status().is_success());

    // 2. Extract Cookie
    let cookie_header = response
        .headers()
        .get("set-cookie")
        .expect("Missing Set-Cookie header in registration response");
    
    let cookie_str = cookie_header.to_str().expect("Failed to convert cookie to string");
    // Extract name=value part (strip attributes like Path, HttpOnly)
    let session_cookie = cookie_str.split(';').next().expect("Failed to parse cookie");

    // 3. Call Logout
    let response = client
        .delete(&logout_url)
        .header("Cookie", session_cookie)
        .header("Content-Type", "application/json")
        .body("{}")
        .send()
        .await
        .expect("Failed to call logout");

    if !response.status().is_success() {
        let response_status = response.status();
        let text = response.text().await.unwrap_or_default();
        panic!("Logout failed. Status: {}, Body: {}", response_status, text);
    }

    let api_response = response
        .json::<ApiResponse<String>>()
        .await
        .expect("Failed to deserialize logout response");

    assert_eq!(api_response.data, Some("Successfully logged out the user".to_string()));
    assert!(api_response.error.is_none());

    // 4. Verify Session Deleted
    let mut result = db
        .query("SELECT * FROM user_identifier WHERE identifier_value = 'logout@example.com'")
        .await
        .expect("Failed to query user");
    
    let user_identifier: Option<merzah::models::user::UserIdentifier> = result.take(0).expect("Failed to parse user");
    let user_id = user_identifier.expect("User not found").user;

    let mut session_result = db
        .query("SELECT * FROM sessions WHERE user = $user")
        .bind(("user", user_id))
        .await
        .expect("Failed to query sessions");
    
    let sessions: Option<merzah::models::session::Session> = session_result.take(0).unwrap();
    assert!(sessions.is_none(), "Session should have been deleted");
}

#[tokio::test]
async fn login_server_fn_successfully_logs_in_user() {
    let client = Client::new();
        
    let db = get_test_db().await;
    let addr = spawn_app(db.clone());
    let register_url = format!("{}/auth/register", addr);
    let login_url = format!("{}/auth/login", addr);
    let logout_url = format!("{}/auth/logout", addr);


    let name = "Login Test User".to_string();
    let email = "login_test@example.com".to_string();
    let password = "password123".to_string();

    let reg_form = RegistrationFormData::new(
        name.clone(),
        Identifier::Email(email.clone()),
        password.clone(),
        Platform::Web,
    );
    let reg_body = RegisterationFormWrapper { form: reg_form };

    let reg_response = client
        .post(&register_url)
        .json(&reg_body)
        .send()
        .await
        .expect("Failed to register");

    

    assert!(reg_response.status().is_success(), "Registration failed: {:?}", reg_response.text().await);

    let cookie_header = reg_response
        .headers()
        .get("set-cookie")
        .expect("Missing Set-Cookie header in registration response");

    let cookie_str = cookie_header.to_str().expect("Failed to convert cookie to string");
    let session_cookie = cookie_str.split(';').next().expect("Failed to parse cookie");

    let logout_client = Client::new();

    let logout_res = logout_client
        .delete(logout_url)
        .header("Cookie", session_cookie)
        .send()
        .await
        .expect("Failed to send request to logout");

    assert!(logout_res.status().is_success());

    let api_response = logout_res
        .json::<ApiResponse<String>>()
        .await
        .expect("Failed to deserialize logout response");

    assert_eq!(api_response.data, Some("Successfully logged out the user".to_string()));
    assert!(api_response.error.is_none());
            
    let login_client = Client::new();

    let login_form = LoginFormData {
        identifier: Identifier::Email(email.clone()),
        password: password.clone(),
        platform: Platform::Web,
    };
    let login_body = LoginFormWrapper { form: login_form };

    let login_response = login_client
        .post(&login_url)
        .json(&login_body)
        .send()
        .await
        .expect("Failed to login");

    if !login_response.status().is_success() {
        let status = login_response.status();
        let text = login_response.text().await.unwrap_or_default();
        panic!("Login failed. Status: {}, Body: {}", status, text);
    }

    let api_response = login_response
        .json::<ApiResponse<String>>()
        .await
        .expect("Failed to deserialize login response");

    assert_eq!(api_response.data, Some("The user has been logged in successfully".to_string()));
    assert!(api_response.error.is_none());



    let mut result = db
        .query("SELECT * FROM user_identifier WHERE identifier_value = $val")
        .bind(("val", email.clone()))
        .await
        .expect("Failed to query user");
    
    let user_identifier: Option<merzah::models::user::UserIdentifier> = result.take(0).expect("Failed to parse user");
    let user_id = user_identifier.expect("User not found").user;

    let mut session_result = db
        .query("SELECT * FROM sessions WHERE user = $user")
        .bind(("user", user_id))
        .await
        .expect("Failed to query sessions");
    
    let sessions: Vec<merzah::models::session::Session> = session_result.take(0).expect("Failed to parse sessions");
    
    assert!(!sessions.is_empty(), "A session should exist for the logged in user");
    assert_eq!(sessions.len(), 1_usize);
}

#[tokio::test]
async fn mobile_auth_flow_works_correctly() {
    let client = Client::new();
    let db = get_test_db().await;
    let addr = spawn_app(db.clone());
    let register_url = format!("{}/auth/register", addr);
    let login_url = format!("{}/auth/login", addr);
    let logout_url = format!("{}/auth/logout", addr);

    let name = "Mobile User".to_string();
    let email = "mobile@example.com".to_string();
    let password = "password123".to_string();

    // 1. Register as Mobile
    let reg_form = RegistrationFormData::new(
        name.clone(),
        Identifier::Email(email.clone()),
        password.clone(),
        Platform::Mobile,
    );
    let reg_body = RegisterationFormWrapper { form: reg_form };

    let response = client
        .post(&register_url)
        .json(&reg_body)
        .send()
        .await
        .expect("Failed to register");

    assert!(response.status().is_success());
    assert!(response.headers().get("set-cookie").is_none(), "Mobile registration should not set cookies");

    let api_response = response
        .json::<ApiResponse<String>>()
        .await
        .expect("Failed to deserialize response");
    
    let session_token = api_response.data.expect("Mobile registration should return session token");
    assert!(!session_token.is_empty());

    // 2. Logout using the token
    let response = client
        .delete(&logout_url)
        .header("Authorization", format!("Bearer {}", session_token))
        .send()
        .await
        .expect("Failed to logout");

    assert!(response.status().is_success());

    // 3. Login as Mobile
    let login_form = LoginFormData {
        identifier: Identifier::Email(email.clone()),
        password: password.clone(),
        platform: Platform::Mobile,
    };
    let login_body = LoginFormWrapper { form: login_form };

    let response = client
        .post(&login_url)
        .json(&login_body)
        .send()
        .await
        .expect("Failed to login");

    assert!(response.status().is_success());
    assert!(response.headers().get("set-cookie").is_none(), "Mobile login should not set cookies");

    let api_response = response
        .json::<ApiResponse<String>>()
        .await
        .expect("Failed to deserialize response");
    
    let new_session_token = api_response.data.expect("Mobile login should return session token");
    assert!(!new_session_token.is_empty());
    assert_ne!(session_token, new_session_token, "New session token should be different");

    // 4. Verify Session exists in DB
    let mut session_result = db
        .query("SELECT * FROM sessions WHERE token = $token")
        .bind(("token", new_session_token))
        .await
        .expect("Failed to query sessions");
    
    let sessions: Vec<merzah::models::session::Session> = session_result.take(0).expect("Failed to parse sessions");
    assert_eq!(sessions.len(), 1);
}
