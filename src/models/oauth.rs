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
impl GoogleTokenResponse {
    pub fn new(access_token: String, expires_in: i64, token_type: String, scope: String) -> Self {
        Self {
            access_token,
            expires_in,
            token_type,
            scope,
        }
    }
}

#[cfg(feature = "ssr")]
#[derive(Debug, Deserialize)]
pub struct GoogleUser {
    pub id: String,
    pub email: String,
    pub name: Option<String>,
    pub picture: Option<String>,
}

#[cfg(feature = "ssr")]
impl GoogleUser {
    pub fn new(id: String, email: String, name: Option<String>, picture: Option<String>) -> Self {
        Self {
            id,
            email,
            name,
            picture,
        }
    }
}
