#[cfg(feature = "ssr")]
use sha2::{Digest, Sha256};

#[cfg(feature = "ssr")]
pub fn hash_session_token(raw_token: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(raw_token.as_bytes());
    hex::encode(hasher.finalize())
}

#[cfg(feature = "ssr")]
pub fn hash_reset_token(raw_token: &str) -> String {
    hash_session_token(raw_token)
}
