use crate::common::get_test_db;
use merzah::models::api_responses::ApiResponse;
use merzah::models::auth::LoginFormData;
use merzah::models::auth::Platform;
use merzah::models::auth::RegistrationFormData;
use merzah::models::password_reset::PasswordChangeRequest;
use merzah::models::password_reset::PasswordResetConfirm;
use merzah::models::password_reset::PasswordResetRequest;
use merzah::models::user::Identifier;
use merzah::spawn_app;
use reqwest::Client;
use serde::Serialize;

#[derive(Serialize)]
struct RegisterWrapper {
    form: RegistrationFormData,
}

impl RegisterWrapper {
    pub fn new(form: RegistrationFormData) -> Self {
        Self { form }
    }
}

#[derive(Serialize)]
struct LoginWrapper {
    form: LoginFormData,
}

impl LoginWrapper {
    pub fn new(form: LoginFormData) -> Self {
        Self { form }
    }
}

#[derive(Serialize)]
struct ChangeWrapper {
    payload: PasswordChangeRequest,
}

impl ChangeWrapper {
    pub fn new(payload: PasswordChangeRequest) -> Self {
        Self { payload }
    }
}

#[derive(Serialize)]
struct ResetRequestWrapper {
    payload: PasswordResetRequest,
}

impl ResetRequestWrapper {
    pub fn new(payload: PasswordResetRequest) -> Self {
        Self { payload }
    }
}

#[derive(Serialize)]
struct ResetConfirmWrapper {
    payload: PasswordResetConfirm,
}

impl ResetConfirmWrapper {
    pub fn new(payload: PasswordResetConfirm) -> Self {
        Self { payload }
    }
}

async fn register_user(
    client: &Client,
    base: &str,
    name: &str,
    identifier: Identifier,
    password: &str,
    platform: Platform,
) -> reqwest::Response {
    let url = format!("{}/auth/register", base);
    let form = RegistrationFormData::new(
        name.to_string(),
        identifier,
        password.to_string(),
        platform,
    );
    let body = RegisterWrapper::new(form);
    client
        .post(&url)
        .json(&body)
        .send()
        .await
        .expect("failed to register")
}

fn session_cookie(response: &reqwest::Response) -> String {
    let header = response
        .headers()
        .get("set-cookie")
        .expect("missing set-cookie")
        .to_str()
        .expect("cookie str");
    header
        .split(';')
        .next()
        .expect("cookie value")
        .to_string()
}

#[tokio::test]
async fn anonymous_login_succeeds_without_prior_session() {
    let client = Client::new();
    let db = get_test_db().await;
    let addr = spawn_app(db);
    let email = format!("anon_{}@example.com", uuid::Uuid::new_v4());
    let reg = register_user(
        &client,
        &addr,
        "Anon",
        Identifier::Email(email.clone()),
        "password123",
        Platform::Web,
    )
    .await;
    assert!(reg.status().is_success());
    let cookie = session_cookie(&reg);
    let logout_url = format!("{}/auth/logout", addr);
    let logout_res = client
        .delete(&logout_url)
        .header("Cookie", cookie)
        .header("Content-Type", "application/json")
        .body("{}")
        .send()
        .await
        .expect("logout");
    assert!(logout_res.status().is_success());

    let login_url = format!("{}/auth/login", addr);
    let form = LoginFormData::new(
        Identifier::Email(email),
        "password123".to_string(),
        Platform::Web,
    );
    let body = LoginWrapper::new(form);
    let login_res = client
        .post(&login_url)
        .json(&body)
        .send()
        .await
        .expect("login");
    assert!(
        login_res.status().is_success(),
        "anonymous login must succeed, got {}",
        login_res.status()
    );
}

#[tokio::test]
async fn duplicate_registration_returns_409() {
    let client = Client::new();
    let db = get_test_db().await;
    let addr = spawn_app(db);
    let email = format!("dup_{}@example.com", uuid::Uuid::new_v4());
    let first = register_user(
        &client,
        &addr,
        "Dup",
        Identifier::Email(email.clone()),
        "password123",
        Platform::Web,
    )
    .await;
    assert!(first.status().is_success());
    let second = register_user(
        &client,
        &addr,
        "Dup",
        Identifier::Email(email),
        "password123",
        Platform::Web,
    )
    .await;
    assert_eq!(second.status().as_u16(), 409);
}

#[tokio::test]
async fn login_with_short_password_returns_422() {
    let client = Client::new();
    let db = get_test_db().await;
    let addr = spawn_app(db);
    let login_url = format!("{}/auth/login", addr);
    let form = LoginFormData::new(
        Identifier::Email("x@example.com".to_string()),
        "short".to_string(),
        Platform::Web,
    );
    let body = LoginWrapper::new(form);
    let res = client
        .post(&login_url)
        .json(&body)
        .send()
        .await
        .expect("login");
    assert_eq!(res.status().as_u16(), 422);
}

#[tokio::test]
async fn wrong_password_returns_generic_401() {
    let client = Client::new();
    let db = get_test_db().await;
    let addr = spawn_app(db);
    let email = format!("wrong_{}@example.com", uuid::Uuid::new_v4());
    let reg = register_user(
        &client,
        &addr,
        "Wrong",
        Identifier::Email(email.clone()),
        "password123",
        Platform::Web,
    )
    .await;
    assert!(reg.status().is_success());
    let login_url = format!("{}/auth/login", addr);
    let form = LoginFormData::new(
        Identifier::Email(email),
        "wrongpass1".to_string(),
        Platform::Web,
    );
    let body = LoginWrapper::new(form);
    let res = client
        .post(&login_url)
        .json(&body)
        .send()
        .await
        .expect("login");
    assert_eq!(res.status().as_u16(), 401);
    let api = res
        .json::<ApiResponse<String>>()
        .await
        .expect("json");
    assert!(api.error.unwrap_or_default().contains("Invalid username or password"));
}

#[tokio::test]
async fn rate_limit_returns_429_after_repeated_failures() {
    let client = Client::new();
    let db = get_test_db().await;
    let addr = spawn_app(db);
    let email = format!("rl_{}@example.com", uuid::Uuid::new_v4());
    let reg = register_user(
        &client,
        &addr,
        "Rl",
        Identifier::Email(email.clone()),
        "password123",
        Platform::Web,
    )
    .await;
    assert!(reg.status().is_success());
    let login_url = format!("{}/auth/login", addr);
    let mut saw_429 = false;
    for _ in 0..7 {
        let form = LoginFormData::new(
            Identifier::Email(email.clone()),
            "wrongpass1".to_string(),
            Platform::Web,
        );
        let body = LoginWrapper::new(form);
        let res = client
            .post(&login_url)
            .json(&body)
            .send()
            .await
            .expect("login");
        if res.status().as_u16() == 429 {
            saw_429 = true;
            break;
        }
    }
    assert!(saw_429, "expected 429 after repeated failures");
}

#[tokio::test]
async fn email_login_is_case_insensitive() {
    let client = Client::new();
    let db = get_test_db().await;
    let addr = spawn_app(db);
    let email = format!("Mixed_{}@Example.COM", uuid::Uuid::new_v4());
    let reg = register_user(
        &client,
        &addr,
        "Mixed",
        Identifier::Email(email.clone()),
        "password123",
        Platform::Web,
    )
    .await;
    assert!(reg.status().is_success());
    let login_url = format!("{}/auth/login", addr);
    let form = LoginFormData::new(
        Identifier::Email(email.to_lowercase()),
        "password123".to_string(),
        Platform::Mobile,
    );
    let body = LoginWrapper::new(form);
    let res = client
        .post(&login_url)
        .json(&body)
        .send()
        .await
        .expect("login");
    assert!(res.status().is_success());
}

#[tokio::test]
async fn change_password_invalidates_old_session() {
    let client = Client::new();
    let db = get_test_db().await;
    let addr = spawn_app(db);
    let email = format!("chg_{}@example.com", uuid::Uuid::new_v4());
    let reg = register_user(
        &client,
        &addr,
        "Chg",
        Identifier::Email(email),
        "password123",
        Platform::Web,
    )
    .await;
    assert!(reg.status().is_success());
    let cookie = session_cookie(&reg);
    let change_url = format!("{}/auth/change-password", addr);
    let payload = PasswordChangeRequest::new("password123".to_string(), "newpassword1".to_string());
    let body = ChangeWrapper::new(payload);
    let res = client
        .post(&change_url)
        .header("Cookie", cookie.clone())
        .json(&body)
        .send()
        .await
        .expect("change");
    assert!(res.status().is_success());
    let me_url = format!("{}/auth/me", addr);
    let me_res = client
        .post(&me_url)
        .header("Cookie", cookie)
        .header("Content-Type", "application/json")
        .body("{}")
        .send()
        .await
        .expect("me");
    assert_eq!(me_res.status().as_u16(), 401, "old session must be invalidated");
}

#[tokio::test]
async fn password_reset_flow_works_end_to_end() {
    let client = Client::new();
    let db = get_test_db().await;
    let addr = spawn_app(db.clone());
    let email = format!("reset_{}@example.com", uuid::Uuid::new_v4());
    let reg = register_user(
        &client,
        &addr,
        "Reset",
        Identifier::Email(email.clone()),
        "password123",
        Platform::Web,
    )
    .await;
    assert!(reg.status().is_success());

    let req_url = format!("{}/auth/request-reset", addr);
    let req_payload = PasswordResetRequest::new("email".to_string(), email.clone());
    let req_body = ResetRequestWrapper::new(req_payload);
    let req_res = client
        .post(&req_url)
        .json(&req_body)
        .send()
        .await
        .expect("request reset");
    assert!(req_res.status().is_success());
    let api = req_res
        .json::<ApiResponse<String>>()
        .await
        .expect("json");
    let raw_token = api.data.expect("reset token returned");

    let confirm_url = format!("{}/auth/confirm-reset", addr);
    let confirm_payload = PasswordResetConfirm::new(raw_token, "brandnew12".to_string());
    let confirm_body = ResetConfirmWrapper::new(confirm_payload);
    let confirm_res = client
        .post(&confirm_url)
        .json(&confirm_body)
        .send()
        .await
        .expect("confirm");
    assert!(confirm_res.status().is_success());

    let login_url = format!("{}/auth/login", addr);
    let form = LoginFormData::new(
        Identifier::Email(email),
        "brandnew12".to_string(),
        Platform::Mobile,
    );
    let body = LoginWrapper::new(form);
    let login_res = client
        .post(&login_url)
        .json(&body)
        .send()
        .await
        .expect("login with new password");
    assert!(login_res.status().is_success());
}

#[tokio::test]
async fn expired_session_cannot_access_me() {
    let client = Client::new();
    let db = get_test_db().await;
    let addr = spawn_app(db.clone());
    let email = format!("exp_{}@example.com", uuid::Uuid::new_v4());
    let reg = register_user(
        &client,
        &addr,
        "Exp",
        Identifier::Email(email.clone()),
        "password123",
        Platform::Mobile,
    )
    .await;
    assert!(reg.status().is_success());
    let api = reg
        .json::<ApiResponse<String>>()
        .await
        .expect("json");
    let raw_token = api.data.expect("token");
    let hashed = merzah::models::session_token::hash_session_token(&raw_token);
    db.query("UPDATE sessions SET expires_at = time::now() - 1h WHERE session_token = $t")
        .bind(("t", hashed))
        .await
        .expect("expire");
    let me_url = format!("{}/auth/me", addr);
    let me_res = client
        .post(&me_url)
        .header("Authorization", format!("Bearer {}", raw_token))
        .header("Content-Type", "application/json")
        .body("{}")
        .send()
        .await
        .expect("me");
    assert_eq!(me_res.status().as_u16(), 401);
}
