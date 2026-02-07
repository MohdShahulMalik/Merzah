# WorkOS Integration Plan

This document outlines the step-by-step process for integrating WorkOS authentication (Social Logins) into the Merzah project while maintaining compatibility with the existing custom session system.

## 1. Prerequisites & Configuration

### A. WorkOS Dashboard
1.  Create a WorkOS account and organization.
2.  Configure **Social Connections** in the WorkOS dashboard (Google, Apple, GitHub, etc.).
3.  For Meta, X, and Instagram, set up **Generic OIDC** connections.
4.  Set the **Redirect URI** to `http://localhost:3000/auth/callback` (for local development).

### B. Environment Variables
Add the following to your server environment:
```bash
WORKOS_API_KEY=sk_test_...
WORKOS_CLIENT_ID=project_...
# connection_id for specific providers if not using AuthKit
WORKOS_GOOGLE_CONNECTION=conn_...
WORKOS_APPLE_CONNECTION=conn_...
```

---

## 2. Database & Model Updates

### A. Schema Update (`schemas/user_identifier.surql`)
Extend the `identifier_type` to include `workos`.
```surql
-- Update the ASSERT clause
DEFINE FIELD IF NOT EXISTS identifier_type ON user_identifier TYPE string 
    ASSERT $value IN ['email', 'mobile', 'workos'];
```

### B. Model Update (`src/models/user.rs`)
Update the `Identifier` enum to include the Workos variant.
```rust
pub enum Identifier {
    #[serde(rename = "email")]
    Email(#[garde(email)] String),
    #[serde(rename = "mobile")]
    Mobile(#[garde(pattern(...))] String),
    #[serde(rename = "workos")]
    Workos(String), // Stores the unique WorkOS User ID
}
```

---

## 3. Backend Implementation (`src/server_functions/auth.rs`)

### A. Core Workflow Module (`src/auth/workos.rs`)
Create a dedicated module for WorkOS logic:
1.  **`get_auth_url(provider: String)`**: Use `workos::sso::Sso::get_authorization_url`.
2.  **`exchange_code(code: String)`**: Use `workos::sso::Sso::get_profile_and_token`.
3.  **`find_or_create_user(profile: Profile)`**:
    *   Search `user_identifier` for `identifier_type = "workos"` and `identifier_value = profile.id`.
    *   If found: Return the linked `user`.
    *   If not found: Create a new `users` record and a `user_identifier` record, then return the new `user`.

### B. Server Functions
Implement two new endpoints:

1.  **`get_social_auth_url(provider: String)`**:
    *   Input: Provider name (e.g., "google").
    *   Output: `ApiResponse<String>` containing the redirect URL.
2.  **`authenticate_social_code(code: String)`**:
    *   Input: Code from the redirect.
    *   Action: 
        *   Exchanges code for profile.
        *   Maps WorkOS profile to local SurrealDB user.
        *   Calls `create_session(user_id, &db)`.
        *   Calls `set_session_cookie(&token)`.
    *   Output: `ApiResponse<String>` success message.

---

## 4. Frontend Integration (Leptos)

### A. Login Page Update
1.  Add social login buttons (e.g., "Continue with Google").
2.  Clicking a button triggers the `get_social_auth_url` server function.
3.  Redirect the user: `window.location.href = url;`.

### B. Callback Page (`src/pages/auth_callback.rs`)
1.  Create a new page/route at `/auth/callback`.
2.  On mount (in a `create_effect` or `create_resource`):
    *   Extract the `code` from the URL query parameters.
    *   Call the `authenticate_social_code(code)` server function.
    *   On success: Redirect to the home screen or dashboard.
    *   On failure: Show an error message.

---

## 5. Security & Refinement

1.  **State Parameter**: Generate a random `state` parameter in `get_social_auth_url` and verify it in the callback to prevent CSRF.
2.  **Session Unification**: Ensure that `get_authenticated_user` continues to work seamlessly because the social login eventually creates a standard session in your `sessions` table.
3.  **Profile Syncing**: Optionally update the user's `display_name` in SurrealDB if the WorkOS profile contains updated information (like a new Google profile picture or name).

---

## 6. Implementation Sequence
1.  [ ] Apply SurrealDB schema changes.
2.  [ ] Update Rust models.
3.  [ ] Add `workos` crate to `Cargo.toml`.
4.  [ ] Implement `src/auth/workos.rs` logic.
5.  [ ] Add server functions to `src/server_functions/auth.rs`.
6.  [ ] Create the Callback page in Leptos.
7.  [ ] Add Social Buttons to the Login UI.
