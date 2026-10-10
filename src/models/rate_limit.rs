#[cfg(feature = "ssr")]
use std::collections::HashMap;
#[cfg(feature = "ssr")]
use std::sync::Mutex;
#[cfg(feature = "ssr")]
use std::time::{Duration, Instant};
#[cfg(feature = "ssr")]
use std::sync::OnceLock;

#[cfg(feature = "ssr")]
pub const MAX_LOGIN_ATTEMPTS: u32 = 5;
#[cfg(feature = "ssr")]
pub const RATE_LIMIT_WINDOW_SECS: u64 = 300;

#[cfg(feature = "ssr")]
struct AttemptInfo {
    count: u32,
    first_attempt: Instant,
}

#[cfg(feature = "ssr")]
static LOGIN_ATTEMPTS: OnceLock<Mutex<HashMap<String, AttemptInfo>>> = OnceLock::new();

#[cfg(feature = "ssr")]
fn attempts_store() -> &'static Mutex<HashMap<String, AttemptInfo>> {
    LOGIN_ATTEMPTS.get_or_init(|| Mutex::new(HashMap::new()))
}

#[cfg(feature = "ssr")]
pub fn check_login_rate_limit(key: &str) -> Result<(), u64> {
    let store = attempts_store();
    let guard = store.lock().expect("rate limit mutex poisoned");
    if let Some(info) = guard.get(key) {
        if info.first_attempt.elapsed() > Duration::from_secs(RATE_LIMIT_WINDOW_SECS) {
            return Ok(());
        }
        if info.count >= MAX_LOGIN_ATTEMPTS {
            let remaining = RATE_LIMIT_WINDOW_SECS.saturating_sub(info.first_attempt.elapsed().as_secs());
            return Err(remaining);
        }
    }
    Ok(())
}

#[cfg(feature = "ssr")]
pub fn record_login_failure(key: &str) {
    let store = attempts_store();
    let mut guard = store.lock().expect("rate limit mutex poisoned");
    let now = Instant::now();
    guard
        .entry(key.to_string())
        .and_modify(|info| {
            if info.first_attempt.elapsed() > Duration::from_secs(RATE_LIMIT_WINDOW_SECS) {
                info.count = 1;
                info.first_attempt = now;
            } else {
                info.count += 1;
            }
        })
        .or_insert(AttemptInfo {
            count: 1,
            first_attempt: now,
        });
}

#[cfg(feature = "ssr")]
pub fn record_login_success(key: &str) {
    let store = attempts_store();
    let mut guard = store.lock().expect("rate limit mutex poisoned");
    guard.remove(key);
}

#[cfg(feature = "ssr")]
pub fn rate_limit_key(identifier_type: &str, identifier_value: &str) -> String {
    format!("{}:{}", identifier_type, identifier_value.to_lowercase())
}

#[cfg(feature = "ssr")]
pub fn clear_rate_limit_for_tests() {
    let store = attempts_store();
    let mut guard = store.lock().expect("rate limit mutex poisoned");
    guard.clear();
}
