# Custom OAuth 2.0 Implementation Plan

This document outlines the step-by-step process for implementing custom OAuth 2.0 authentication in the Merzah project, without relying on WorkOS.

---

## Overview

### Providers to Support
- **Google** (Phase 1) - Recommended, easiest to implement
- **Meta/Facebook** (Phase 2)
- **Instagram** (Phase 3)

### Prerequisites (Per Provider)

Register apps with each provider to get credentials:

| Provider | Developer Portal | Key Variables |
|----------|------------------|---------------|
| Google | console.cloud.google.com | `GOOGLE_CLIENT_ID`, `GOOGLE_CLIENT_SECRET` |
| Meta/Facebook | developers.facebook.com | `META_CLIENT_ID`, `META_CLIENT_SECRET` |
| Instagram | Same as Meta | Uses Meta app |

---

## Phase 1: Google OAuth

### Step 1: Database Schema Update

**File:** `schemas/user_identifier.surql`

Change line 4 to:
```surql
DEFINE FIELD IF NOT EXISTS identifier_type ON user_identifier TYPE string 
    ASSERT $value IN ['email', 'mobile', 'google', 'meta', 'instagram'];
```

### Step 2: Update User Model

**File:** `src/models/user.rs`

Update the `Identifier` enum:
```rust
#[derive(Debug, Validate, Deserialize, Serialize, Clone)]
#[serde(tag = "identifier_type", content = "identifier_value")]
pub enum Identifier {
    #[serde(rename = "email")]
    Email(#[garde(email)] String),
    #[serde(rename = "mobile")]
    Mobile(#[garde(pattern(r"^[+]?[(]?[0-9]{1,4}[)]?[- .]?[(]?[0-9]{1,4}[)]?[- .]?[0-9]{4,10}$"))] String),
    #[serde(rename = "google")]
    Google(#[garde(skip)] String),
    #[serde(rename = "meta")]
    Meta(#[garde(skip)] String),
    #[serde(rename = "instagram")]
    Instagram(#[garde(skip)] String),
}
```

### Step 3: Update Auth Model

**File:** `src/models/auth.rs`

- Update `validate_uniqueness` method to handle Google/Meta/Instagram variants
- Remove `Workos` references

### Step 4: Update Custom Auth

**File:** `src/auth/custom_auth.rs`

- Update `authenticate` function to handle Google/Meta/Instagram variants (return error - they can't login with password)

### Step 5: Create OAuth Module

**New File:** `src/auth/oauth/mod.rs`

```rust
#[cfg(feature = "ssr")]
pub mod google;
#[cfg(feature = "ssr")]
pub mod state;
```

### Step 6: Create State Management (CSRF Protection)

**New File:** `src/auth/oauth/state.rs`

```rust
use base64::{Engine as _, engine::general_purpose};
use rand::{Rng, thread_rng};

pub fn generate_state() -> String {
    let mut bytes = [0u8; 32];
    thread_rng().fill(&mut bytes);
    general_purpose::URL_SAFE_NO_PAD.encode(bytes)
}

pub fn validate_state(state: &str, stored_state: &str) -> bool {
    state == stored_state && !state.is_empty()
}
```

### Step 7: Create Google OAuth Implementation

**New File:** `src/auth/oauth/google.rs`

```rust
use serde::{Deserialize, Serialize};
use surrealdb::{RecordId, Surreal};
use surrealdb::engine::remote::ws::Client;
use crate::models::user::{CreateUser, Identifier};
use crate::utils::token_generator::generate_token;

#[derive(Debug, Deserialize)]
pub struct GoogleTokenResponse {
    pub access_token: String,
    pub expires_in: i64,
    pub token_type: String,
    pub scope: String,
}

#[derive(Debug, Deserialize)]
pub struct GoogleUser {
    pub id: String,
    pub email: String,
    pub name: Option<String>,
    pub picture: Option<String>,
}

pub fn get_authorization_url(state: &str) -> String {
    let client_id = std::env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID must be set");
    let redirect_uri = std::env::var("GOOGLE_REDIRECT_URI").expect("GOOGLE_REDIRECT_URI must be set");
    
    let params = [
        ("client_id", client_id),
        ("redirect_uri", redirect_uri),
        ("response_type", "code".to_string()),
        ("scope", "openid email profile".to_string()),
        ("state", state.to_string()),
    ];
    
    let url = reqwest::Url::parse_with_params(
        "https://accounts.google.com/o/oauth2/v2/auth",
        &params
    ).unwrap();
    
    url.to_string()
}

pub async fn exchange_code(code: &str) -> Result<GoogleTokenResponse, Box<dyn std::error::Error>> {
    let client_id = std::env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID must be set");
    let client_secret = std::env::var("GOOGLE_CLIENT_SECRET").expect("GOOGLE_CLIENT_SECRET must be set");
    let redirect_uri = std::env::var("GOOGLE_REDIRECT_URI").expect("GOOGLE_REDIRECT_URI must be set");
    
    let client = reqwest::Client::new();
    
    let response = client
        .post("https://oauth2.googleapis.com/token")
        .form(&[
            ("client_id", client_id.as_str()),
            ("client_secret", client_secret.as_str()),
            ("code", code),
            ("grant_type", "authorization_code"),
            ("redirect_uri", redirect_uri.as_str()),
        ])
        .send()
        .await?;
    
    let token_response: GoogleTokenResponse = response.json().await?;
    Ok(token_response)
}

pub async fn get_user_info(access_token: &str) -> Result<GoogleUser, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    
    let response = client
        .get("https://www.googleapis.com/oauth2/v2/userinfo")
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await?;
    
    let user: GoogleUser = response.json().await?;
    Ok(user)
}

pub async fn find_or_create_user(
    profile: GoogleUser,
    db: &Surreal<Client>
) -> Result<RecordId, Box<dyn std::error::Error>> {
    // Check if user already exists with this Google ID
    let existing: Option<serde_json::Value> = db
        .query("SELECT user FROM user_identifier WHERE identifier_type = 'google' AND identifier_value = $id")
        .bind(("id", profile.id.clone()))
        .await?
        .take(0)?;
    
    if let Some(record) = existing {
        let user_id: RecordId = record.get("user").unwrap().as_record_id().unwrap();
        return Ok(user_id);
    }
    
    // Create new user
    let display_name = profile.name.unwrap_or_else(|| profile.email.split('@').next().unwrap_or("User").to_string());
    
    let user = CreateUser {
        display_name,
        password_hash: format!("oauth_google_{}", generate_token()), // Placeholder
    };
    
    let surql = r#"
        BEGIN TRANSACTION;
        
        LET $created_user = (CREATE ONLY users CONTENT $user_data);
        
        CREATE user_identifier CONTENT {
            user: $created_user.id,
            identifier_type: 'google',
            identifier_value: $provider_id
        };
        
        RETURN $created_user;
        COMMIT TRANSACTION;
    "#;
    
    let result = db.query(surql)
        .bind(("user_data", user))
        .bind(("provider_id", profile.id))
        .await?;
    
    let created_user: Option<CreateUser> = result.take(0)?;
    Ok(created_user.unwrap().id)
}
```

### Step 8: Add Environment Variables

**File:** `.env`

```bash
GOOGLE_CLIENT_ID=your_google_client_id
GOOGLE_CLIENT_SECRET=your_google_client_secret
GOOGLE_REDIRECT_URI=http://localhost:3000/auth/callback/google
```

### Step 9: Add Server Functions

**File:** `src/server_functions/auth.rs`

Add two new endpoints:

```rust
#[server(input=Json, prefix="/auth", endpoint="google-url")]
pub async fn get_google_oauth_url() -> Result<ApiResponse<String>, ServerFnError> {
    use crate::auth::oauth::google::get_authorization_url;
    use crate::auth::oauth::state::generate_state;
    
    let state = generate_state();
    let url = get_authorization_url(&state);
    
    // Store state in cookie for validation
    let response = expect_context::<leptos_actix::ResponseOptions>();
    let cookie = format!(
        "oauth_state={}; Path=/; Secure; HttpOnly; SameSite=Lax; Max-Age={}",
        state,
        10 * 60 // 10 minutes
    );
    response.insert_header(
        SET_COOKIE,
        HeaderValue::from_str(&cookie).unwrap()
    );
    
    Ok(ApiResponse { data: Some(url), error: None })
}

#[server(input=Json, prefix="/auth", endpoint="google-callback")]
pub async fn handle_google_callback(
    code: String,
    state: String,
) -> Result<ApiResponse<String>, ServerFnError> {
    use crate::auth::oauth::google::{exchange_code, get_user_info, find_or_create_user};
    use crate::auth::oauth::state::validate_state;
    
    let (response_option, db) = match get_server_context().await {
        Ok(ctx) => ctx,
        Err(e) => return Ok(e),
    };
    
    // Validate state
    // (In real implementation, read from cookie and validate)
    
    // Exchange code for token
    let token_response = match exchange_code(&code).await {
        Ok(token) => token,
        Err(e) => {
            error!(?e, "Failed to exchange code");
            response_option.set_status(StatusCode::BAD_REQUEST);
            return Ok(ApiResponse::error("Failed to exchange code".to_string()));
        }
    };
    
    // Get user info
    let user_info = match get_user_info(&token_response.access_token).await {
        Ok(user) => user,
        Err(e) => {
            error!(?e, "Failed to get user info");
            response_option.set_status(StatusCode::BAD_REQUEST);
            return Ok(ApiResponse::error("Failed to get user info".to_string()));
        }
    };
    
    // Find or create user
    let user_id = match find_or_create_user(user_info, &db).await {
        Ok(id) => id,
        Err(e) => {
            error!(?e, "Failed to find or create user");
            return Err(ServerFnError::ServerError("Failed to authenticate user".to_string()));
        }
    };
    
    // Create session
    let session_token = match create_session(user_id, &db).await {
        Ok(token) => token,
        Err(e) => {
            error!(?e, "Failed to create session");
            return Err(ServerFnError::ServerError("Failed to create session".to_string()));
        }
    };
    
    // Set session cookie
    if let Err(e) = set_session_cookie(&session_token) {
        error!(?e, "Failed to set session cookie");
        return Err(ServerFnError::ServerError("Failed to set session".to_string()));
    }
    
    // Clear oauth state cookie
    let clear_cookie = "oauth_state=; Path=/; Secure; HttpOnly; SameSite=Lax; Max-Age=0";
    response_option.insert_header(
        SET_COOKIE,
        HeaderValue::from_str(clear_cookie).unwrap()
    );
    
    Ok(ApiResponse::data("Successfully authenticated with Google".to_string()))
}
```

### Step 10: Add Callback Route

**File:** `src/app.rs`

Add new route:
```rust
<Route path=path!("/auth/callback/google") view=GoogleCallback/>
```

**New File:** `src/pages/google_callback.rs`

```rust
use leptos::{prelude::*, reactive::spawn_local};
use leptos_router::components::A;
use crate::server_functions::auth::handle_google_callback;

#[component]
pub fn GoogleCallback() -> impl IntoView {
    let (error, set_error) = signal(String::new());
    
    // Get URL params on mount
    let params = use_params_map();
    
    let code = move || params.get("code").cloned().unwrap_or_default();
    let state = move || params.get("state").cloned().unwrap_or_default();
    
    // Process OAuth callback on mount
    create_effect(move |_| {
        let code = code();
        let state = state();
        
        if !code.is_empty() {
            spawn_local(async move {
                match handle_google_callback(code, state).await {
                    Ok(response) => {
                        if response.error.is_some() {
                            set_error.set(response.error.unwrap());
                        } else {
                            // Redirect to home on success
                            window().location().set_href("/").ok();
                        }
                    }
                    Err(e) => {
                        set_error.set(format!("Authentication failed: {}", e));
                    }
                }
            });
        }
    });
    
    view! {
        <div>
            <Show when=move || error.get().is_empty() fallback=move || view! { <p>{error.get()}</p> }>
                <p>"Authenticating with Google..."</p>
            </Show>
        </div>
    }
}
```

### Step 11: Add Login Button

**File:** `src/pages/auth.rs` (Login component)

Add Google login button:
```rust
use crate::server_functions::auth::get_google_oauth_url;

let start_google_oauth = move |_| {
    spawn_local(async move {
        match get_google_oauth_url().await {
            Ok(response) => {
                if let Some(url) = response.data {
                    window().location().set_href(&url).ok();
                }
            }
            Err(e) => {
                error.set(format!("Failed to start Google login: {}", e));
            }
        }
    });
};

// In the view, add the button:
<button 
    on:click=start_google_oauth
    class="flex items-center justify-center gap-2"
>
    <img src="/assets/google-icon.png" alt="Google" class="w-5 h-5" />
    "Continue with Google"
</button>
```

---

## Phase 2: Meta/Facebook OAuth

Same structure as Phase 1, with the following differences:

### New File: `src/auth/oauth/meta.rs`

```rust
pub fn get_authorization_url(state: &str) -> String {
    let client_id = std::env::var("META_CLIENT_ID").expect("META_CLIENT_ID must be set");
    let redirect_uri = std::env::var("META_REDIRECT_URI").expect("META_REDIRECT_URI must be set");
    
    let params = [
        ("client_id", client_id),
        ("redirect_uri", redirect_uri),
        ("response_type", "code".to_string()),
        ("scope", "public_profile,email".to_string()),
        ("state", state.to_string()),
    ];
    
    let url = reqwest::Url::parse_with_params(
        "https://www.facebook.com/v18.0/dialog/oauth",
        &params
    ).unwrap();
    
    url.to_string()
}

pub async fn exchange_code(code: &str) -> Result<MetaTokenResponse, Box<dyn std::error::Error>> {
    let client_id = std::env::var("META_CLIENT_ID").expect("META_CLIENT_ID must be set");
    let client_secret = std::env::var("META_CLIENT_SECRET").expect("META_CLIENT_SECRET must be set");
    let redirect_uri = std::env::var("META_REDIRECT_URI").expect("META_REDIRECT_URI must be set");
    
    let client = reqwest::Client::new();
    
    let response = client
        .get("https://graph.facebook.com/v18.0/oauth/access_token")
        .query(&[
            ("client_id", client_id.as_str()),
            ("client_secret", client_secret.as_str()),
            ("code", code),
            ("redirect_uri", redirect_uri.as_str()),
        ])
        .send()
        .await?;
    
    let token_response: MetaTokenResponse = response.json().await?;
    Ok(token_response)
}

pub async fn get_user_info(access_token: &str) -> Result<MetaUser, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    
    let response = client
        .get("https://graph.facebook.com/me")
        .query(&[
            ("fields", "id,name,email,picture"),
            ("access_token", access_token),
        ])
        .send()
        .await?;
    
    let user: MetaUser = response.json().await?;
    Ok(user)
}
```

### Environment Variables

```bash
META_CLIENT_ID=your_meta_app_id
META_CLIENT_SECRET=your_meta_app_secret
META_REDIRECT_URI=http://localhost:3000/auth/callback/meta
```

---

## Phase 3: Instagram OAuth

Uses the same Meta app but with Instagram-specific endpoints.

### New File: `src/auth/oauth/instagram.rs`

```rust
pub async fn get_user_info(access_token: &str) -> Result<InstagramUser, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    
    let response = client
        .get("https://graph.instagram.com/me")
        .query(&[
            ("fields", "id,username,account_type,media_count"),
            ("access_token", access_token),
        ])
        .send()
        .await?;
    
    let user: InstagramUser = response.json().await?;
    Ok(user)
}
```

### Environment Variables

```bash
INSTAGRAM_CLIENT_ID=your_instagram_app_id
INSTAGRAM_CLIENT_SECRET=your_instagram_app_secret
INSTAGRAM_REDIRECT_URI=http://localhost:3000/auth/callback/instagram
```

---

## Summary of Files

| Phase | Files to Create | Files to Modify |
|-------|-----------------|------------------|
| 1 | `src/auth/oauth/mod.rs`, `src/auth/oauth/state.rs`, `src/auth/oauth/google.rs`, `src/pages/google_callback.rs` | `schemas/user_identifier.surql`, `src/models/user.rs`, `src/models/auth.rs`, `src/auth/custom_auth.rs`, `src/server_functions/auth.rs`, `src/app.rs`, `src/pages/auth.rs` |
| 2 | `src/auth/oauth/meta.rs`, `src/pages/meta_callback.rs` | Same structure as Phase 1 |
| 3 | `src/auth/oauth/instagram.rs`, `src/pages/instagram_callback.rs` | Same structure as Phase 1 |

---

## Implementation Order

1. **Phase 1**: Google OAuth
   - Database schema update
   - Model updates
   - OAuth module creation
   - Server functions
   - Frontend integration

2. **Phase 2**: Meta/Facebook OAuth (same pattern)

3. **Phase 3**: Instagram OAuth (same pattern)

---

## Security Considerations

1. **State Parameter**: Always use CSRF state parameter to prevent attacks
2. **Token Storage**: Store OAuth tokens securely (or don't store - use only for initial auth)
3. **Scope Minimization**: Request only necessary scopes
4. **Redirect URI Validation**: Ensure redirect URIs match exactly what's registered
5. **PKCE**: Consider adding PKCE for additional security (recommended for production)
