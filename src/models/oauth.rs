#[cfg(feature = "ssr")]
use serde::Deserialize;

#[cfg(feature = "ssr")]
#[derive(Debug, Deserialize)]
pub struct GoogleTokenResponse {
    pub access_token: String,
    pub expires_in: i64,
    pub token_type: String,
    pub scope: String,
}

#[cfg(feature = "ssr")]
#[derive(Debug, Deserialize)]
pub struct GoogleUser {
    pub id: String,
    pub email: String,
    pub name: Option<String>,
    pub picture: Option<String>,
}
