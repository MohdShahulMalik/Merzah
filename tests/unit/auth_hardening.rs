use crate::common::get_test_db;
use merzah::auth::custom_auth::authenticate;
use merzah::auth::custom_auth::change_password;
use merzah::auth::custom_auth::confirm_password_reset;
use merzah::auth::custom_auth::hash_password;
use merzah::auth::custom_auth::register_user;
use merzah::auth::custom_auth::request_password_reset;
use merzah::auth::custom_auth::validate_password_policy;
use merzah::auth::oauth::google::find_or_create_user;
use merzah::auth::session::cleanup_expired_sessions;
use merzah::auth::session::create_session;
use merzah::auth::session::delete_session;
use merzah::auth::session::extract_bearer_token;
use merzah::auth::session::get_user_by_session;
use merzah::auth::session::validate_session_token;
use merzah::errors::auth::AuthError;
use merzah::models::auth::LoginFormData;
use merzah::models::auth::Platform;
use merzah::models::auth::RegistrationFormData;
use merzah::models::oauth::GoogleUser;
use merzah::models::rate_limit::check_login_rate_limit;
use merzah::models::rate_limit::clear_rate_limit_for_tests;
use merzah::models::rate_limit::rate_limit_key;
use merzah::models::rate_limit::record_login_failure;
use merzah::models::rate_limit::record_login_success;
use merzah::models::session_token::hash_session_token;
use merzah::models::user::Identifier;
use merzah::models::user::normalize_email;
use merzah::models::user::normalize_mobile;
use rstest::rstest;

#[rstest]
#[case::too_short("short", false)]
#[case::exactly_min("12345678", true)]
#[case::normal("thisisasecret", true)]
fn password_policy_enforces_min_and_max(#[case] password: &str, #[case] expected_ok: bool) {
    let result = validate_password_policy(password);
    assert_eq!(result.is_ok(), expected_ok, "password: len={}", password.len());
}

#[test]
fn password_policy_rejects_overlong() {
    let long = "a".repeat(129);
    assert!(validate_password_policy(&long).is_err());
    let max = "a".repeat(128);
    assert!(validate_password_policy(&max).is_ok());
}

#[rstest]
#[case::upper("  FOO@Example.COM ", "foo@example.com")]
#[case::already_lower("a@b.co", "a@b.co")]
fn email_normalization_lowercases_and_trims(#[case] raw: &str, #[case] expected: &str) {
    assert_eq!(normalize_email(raw), expected);
}

#[rstest]
#[case::spaces("+91 12345 67890", "+911234567890")]
#[case::dashes("+1-555-123-4567", "+15551234567")]
#[case::parens("+1 (555) 123 4567", "+15551234567")]
fn mobile_normalization_strips_separators(#[case] raw: &str, #[case] expected: &str) {
    assert_eq!(normalize_mobile(raw), expected);
}

#[test]
fn identifier_normalized_and_oauth_rejected_for_password_login() {
    let email = Identifier::Email("  FOO@X.com ".to_string());
    let normalized = email.normalized();
    assert_eq!(normalized.identifier_value(), "foo@x.com");
    assert!(normalized.is_password_login_allowed());

    let google = Identifier::Google("123".to_string());
    assert!(!google.is_password_login_allowed());

    let discord = Identifier::Discord("123".to_string());
    assert!(!discord.is_password_login_allowed());

    let microsoft = Identifier::Microsoft("123".to_string());
    assert!(!microsoft.is_password_login_allowed());
}

#[test]
fn hash_password_produces_argon2_verifiable_hash() {
    let hash = hash_password("thisisasecret").expect("should hash");
    assert!(hash.starts_with("$argon2"));
    assert!(validate_password_policy("thisisasecret").is_ok());
}

#[test]
fn session_token_hash_is_deterministic_64_hex() {
    let raw = "a".repeat(43);
    let h1 = hash_session_token(&raw);
    let h2 = hash_session_token(&raw);
    assert_eq!(h1, h2);
    assert_eq!(h1.len(), 64);
    assert!(h1.chars().all(|c| c.is_ascii_hexdigit()));
    let other = hash_session_token(&"b".repeat(43));
    assert_ne!(h1, other);
}

#[rstest]
#[case::valid("Bearer abc123", Some("abc123"))]
#[case::missing_prefix("abc123", None)]
#[case::empty_bearer("Bearer ", None)]
#[case::empty("", None)]
#[case::double_prefix("Bearer Bearer abc", Some("Bearer abc"))]
fn bearer_extraction_uses_single_prefix(#[case] header: &str, #[case] expected: Option<&str>) {
    let result = extract_bearer_token(header);
    assert_eq!(result.as_deref(), expected);
}

#[rstest]
#[case::too_short("short", false)]
#[case::too_long("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", false)]
#[case::invalid_chars("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa!", false)]
fn session_token_validation_rejects_malformed(#[case] token: &str, #[case] expected_ok: bool) {
    assert_eq!(validate_session_token(token).is_ok(), expected_ok);
}

#[test]
fn session_token_validation_accepts_43() {
    let token = "a".repeat(43);
    assert!(validate_session_token(&token).is_ok());
}

#[test]
fn rate_limiter_blocks_after_five_failures_and_clears_on_success() {
    clear_rate_limit_for_tests();
    let key = rate_limit_key("email", &format!("ratelimit_{}@example.com", uuid::Uuid::new_v4()));
    for _ in 0..5 {
        assert!(check_login_rate_limit(&key).is_ok());
        record_login_failure(&key);
    }
    assert!(check_login_rate_limit(&key).is_err());
    record_login_success(&key);
    assert!(check_login_rate_limit(&key).is_ok());
    clear_rate_limit_for_tests();
}

#[tokio::test]
async fn login_validation_rejects_short_password() {
    clear_rate_limit_for_tests();
    let db = get_test_db().await;
    let form = LoginFormData::new(
        Identifier::Email("nobody@example.com".to_string()),
        "short".to_string(),
        Platform::Web,
    );
    let result = authenticate(form, &db).await;
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.downcast_ref::<AuthError>().is_some());
}

#[tokio::test]
async fn authenticate_rejects_oauth_identifiers() {
    let db = get_test_db().await;
    let form = LoginFormData::new(
        Identifier::Google("some-oauth-id".to_string()),
        "thisisasecret".to_string(),
        Platform::Web,
    );
    let result = authenticate(form, &db).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn duplicate_registration_case_insensitive_conflicts() {
    let db = get_test_db().await;
    let email = format!("Case_{}@example.com", uuid::Uuid::new_v4());
    let first = RegistrationFormData::new(
        "User One".to_string(),
        Identifier::Email(email.clone()),
        "password123".to_string(),
        Platform::Web,
    );
    register_user(first, &db).await.expect("first should succeed");
    let second = RegistrationFormData::new(
        "User Two".to_string(),
        Identifier::Email(email.to_lowercase()),
        "password123".to_string(),
        Platform::Web,
    );
    let result = register_user(second, &db).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn identifier_type_filter_prevents_cross_type_login() {
    let db = get_test_db().await;
    let mobile = format!("+1555{}", rand::random::<u32>() % 900000 + 100000);
    let reg = RegistrationFormData::new(
        "Type Test".to_string(),
        Identifier::Mobile(mobile.clone()),
        "password123".to_string(),
        Platform::Web,
    );
    register_user(reg, &db).await.expect("register mobile");
    let login_as_email = LoginFormData::new(
        Identifier::Email(mobile.clone()),
        "password123".to_string(),
        Platform::Web,
    );
    let result = authenticate(login_as_email, &db).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn session_is_stored_hashed_not_raw() {
    let db = get_test_db().await;
    let email = format!("hash_{}@example.com", uuid::Uuid::new_v4());
    let reg = RegistrationFormData::new(
        "Hash Test".to_string(),
        Identifier::Email(email.clone()),
        "password123".to_string(),
        Platform::Web,
    );
    let user_id = register_user(reg, &db).await.expect("register");
    let raw = create_session(user_id, &db).await.expect("session");
    let mut raw_lookup = db
        .query("SELECT * FROM sessions WHERE session_token = $t")
        .bind(("t", raw.clone()))
        .await
        .expect("query");
    let raw_rows: Vec<merzah::models::session::Session> = raw_lookup.take(0).expect("take");
    assert!(raw_rows.is_empty(), "raw token must not be stored");
    let hashed = hash_session_token(&raw);
    let mut hash_lookup = db
        .query("SELECT * FROM sessions WHERE session_token = $t")
        .bind(("t", hashed))
        .await
        .expect("query");
    let hash_rows: Vec<merzah::models::session::Session> = hash_lookup.take(0).expect("take");
    assert_eq!(hash_rows.len(), 1);
    let user = get_user_by_session(&raw, &db).await.expect("lookup by raw");
    assert_eq!(user.display_name, "Hash Test");
}

#[tokio::test]
async fn delete_expired_session_removes_row_but_reports_expired() {
    let db = get_test_db().await;
    let email = format!("exp_{}@example.com", uuid::Uuid::new_v4());
    let reg = RegistrationFormData::new(
        "Exp Test".to_string(),
        Identifier::Email(email),
        "password123".to_string(),
        Platform::Web,
    );
    let user_id = register_user(reg, &db).await.expect("register");
    let raw = create_session(user_id.clone(), &db).await.expect("session");
    let hashed = hash_session_token(&raw);
    db.query("UPDATE sessions SET expires_at = time::now() - 1h WHERE session_token = $t")
        .bind(("t", hashed.clone()))
        .await
        .expect("expire");
    let result = delete_session(&raw, &db).await;
    assert!(result.is_err());
    let mut check = db
        .query("SELECT * FROM sessions WHERE session_token = $t")
        .bind(("t", hashed))
        .await
        .expect("query");
    let rows: Vec<merzah::models::session::Session> = check.take(0).expect("take");
    assert!(rows.is_empty(), "expired session row must be deleted");
}

#[tokio::test]
async fn cleanup_removes_only_expired_sessions() {
    let db = get_test_db().await;
    let email = format!("clean_{}@example.com", uuid::Uuid::new_v4());
    let reg = RegistrationFormData::new(
        "Clean Test".to_string(),
        Identifier::Email(email),
        "password123".to_string(),
        Platform::Web,
    );
    let user_id = register_user(reg, &db).await.expect("register");
    let raw_valid = create_session(user_id.clone(), &db).await.expect("valid");
    let raw_expired = create_session(user_id.clone(), &db).await.expect("expired");
    let hashed_expired = hash_session_token(&raw_expired);
    db.query("UPDATE sessions SET expires_at = time::now() - 1h WHERE session_token = $t")
        .bind(("t", hashed_expired))
        .await
        .expect("expire one");
    cleanup_expired_sessions(&db).await.expect("cleanup");
    assert!(get_user_by_session(&raw_valid, &db).await.is_ok());
    assert!(get_user_by_session(&raw_expired, &db).await.is_err());
}

#[tokio::test]
async fn change_password_requires_current_and_invalidates_sessions() {
    let db = get_test_db().await;
    let email = format!("chg_{}@example.com", uuid::Uuid::new_v4());
    let reg = RegistrationFormData::new(
        "Chg Test".to_string(),
        Identifier::Email(email.clone()),
        "password123".to_string(),
        Platform::Web,
    );
    let user_id = register_user(reg, &db).await.expect("register");
    let raw = create_session(user_id.clone(), &db).await.expect("session");
    let wrong = change_password(user_id.clone(), "wrongpass1", "newpassword1", &db).await;
    assert!(wrong.is_err());
    change_password(user_id.clone(), "password123", "newpassword1", &db)
        .await
        .expect("change should succeed");
    assert!(get_user_by_session(&raw, &db).await.is_err(), "old session invalidated");
    let login = LoginFormData::new(
        Identifier::Email(email),
        "newpassword1".to_string(),
        Platform::Web,
    );
    authenticate(login, &db).await.expect("new password works");
}

#[tokio::test]
async fn password_reset_single_use_flow() {
    let db = get_test_db().await;
    let email = format!("rst_{}@example.com", uuid::Uuid::new_v4());
    let reg = RegistrationFormData::new(
        "Rst Test".to_string(),
        Identifier::Email(email.clone()),
        "password123".to_string(),
        Platform::Web,
    );
    register_user(reg, &db).await.expect("register");
    let ident = Identifier::Email(email.clone());
    let raw_token = request_password_reset(ident, &db).await.expect("request");
    assert!(!raw_token.is_empty());
    confirm_password_reset(&raw_token, "brandnew12", &db)
        .await
        .expect("confirm");
    let reuse = confirm_password_reset(&raw_token, "another12x", &db).await;
    assert!(reuse.is_err());
    let login = LoginFormData::new(
        Identifier::Email(email),
        "brandnew12".to_string(),
        Platform::Web,
    );
    authenticate(login, &db).await.expect("reset password works");
}

#[tokio::test]
async fn oauth_placeholder_password_is_argon2_hash() {
    let db = get_test_db().await;
    let profile = GoogleUser::new(
        format!("google-{}", uuid::Uuid::new_v4()),
        format!("oauth_{}@example.com", uuid::Uuid::new_v4()),
        Some("OAuth User".to_string()),
        None,
    );
    let user_id = find_or_create_user(profile, &db).await.expect("oauth create");
    let user: Option<merzah::models::user::User> = db.select(user_id).await.expect("select");
    let user = user.expect("user exists");
    assert!(user.password_hash.starts_with("$argon2"), "placeholder must be hashed");
}
