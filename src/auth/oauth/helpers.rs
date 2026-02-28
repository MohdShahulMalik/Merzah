use actix_web::http::StatusCode;
use leptos::prelude::ServerFnError;

use crate::auth::oauth::provider::OAuthProvider;
use crate::auth::oauth::state::{generate_state, validate_state};
use crate::auth::session::create_session;
use crate::models::api_responses::ApiResponse;
use crate::utils::ssr::get_server_context;
use tracing::error;

#[derive(Clone, Copy)]
pub struct OAuthCallback;

impl OAuthCallback {
    pub async fn get_url<P: OAuthProvider + Default + 'static>(
        cookie_name: &str,
    ) -> Result<ApiResponse<String>, ServerFnError> {
        let (response_option, _db) = match get_server_context().await {
            Ok(ctx) => ctx,
            Err(e) => return Ok(e),
        };

        let state = match generate_state() {
            Ok(s) => s,
            Err(e) => {
                error!(?e, "Failed to generate state");
                response_option.set_status(StatusCode::INTERNAL_SERVER_ERROR);
                return Ok(ApiResponse::error("Failed to generate authentication state".to_string()));
            }
        };

        let provider = P::default();
        let url = match provider.authorization_url(&state) {
            Ok(u) => u,
            Err(e) => {
                error!(error = %e, "Failed to get authorization URL");
                response_option.set_status(StatusCode::INTERNAL_SERVER_ERROR);
                return Ok(ApiResponse::error(format!("Failed to create authorization URL: {}", e)));
            }
        };

        let cookie = format!(
            "{}={}; Path=/; Secure; HttpOnly; SameSite=Lax; Max-Age={}",
            cookie_name,
            state,
            10 * 60
        );

        use actix_web::http::header::{HeaderValue, SET_COOKIE};

        let header_value = match HeaderValue::from_str(&cookie) {
            Ok(v) => v,
            Err(e) => {
                error!(?e, "Failed to create header value");
                response_option.set_status(StatusCode::INTERNAL_SERVER_ERROR);
                return Ok(ApiResponse::error("Failed to set cookie".to_string()));
            }
        };

        response_option.insert_header(SET_COOKIE, header_value);

        Ok(ApiResponse { data: Some(url), error: None })
    }

    pub async fn handle<P: OAuthProvider + Default + 'static>(
        code: String,
        state: String,
        cookie_name: &str,
    ) -> Result<ApiResponse<String>, ServerFnError> {
        let (response_option, db) = match get_server_context().await {
            Ok(ctx) => ctx,
            Err(e) => return Ok(e),
        };

        let req = match leptos_actix::extract::<actix_web::HttpRequest>().await {
            Ok(req) => req,
            Err(e) => {
                error!(?e, "Failed to extract request");
                response_option.set_status(StatusCode::INTERNAL_SERVER_ERROR);
                return Ok(ApiResponse::error("Internal server error".to_string()));
            }
        };

        let stored_state = req
            .cookie(cookie_name)
            .map(|c| c.value().to_string())
            .unwrap_or_default();

        if !validate_state(&state, &stored_state) {
            error!("State validation failed");
            response_option.set_status(StatusCode::BAD_REQUEST);
            return Ok(ApiResponse::error("Invalid authentication state".to_string()));
        }

        let provider = P::default();

        let token_response = match provider.exchange_code(&code).await {
            Ok(token) => token,
            Err(e) => {
                error!(error = %e, "Failed to exchange code");
                response_option.set_status(StatusCode::BAD_REQUEST);
                return Ok(ApiResponse::error(format!("Failed to exchange authorization code: {}", e)));
            }
        };

        let user_info = match provider.get_user_info(&token_response.access_token).await {
            Ok(user) => user,
            Err(e) => {
                error!(error = %e, "Failed to get user info");
                response_option.set_status(StatusCode::BAD_REQUEST);
                return Ok(ApiResponse::error(format!("Failed to get user information: {}", e)));
            }
        };

        let user_id = match provider.find_or_create_user(user_info, &db).await {
            Ok(id) => id,
            Err(e) => {
                error!(error = %e, "Failed to find or create user");
                return Err(ServerFnError::ServerError(format!("Failed to authenticate user: {:?}", e)));
            }
        };

        let session_token = match create_session(user_id, &db).await {
            Ok(token) => token,
            Err(e) => {
                error!(?e, "Failed to create session");
                return Err(ServerFnError::ServerError("Failed to create session".to_string()));
            }
        };

        use actix_web::http::header::{HeaderValue, SET_COOKIE};

        let session_cookie = format!(
            "__Host-session={}; Path=/; Secure; HttpOnly; SameSite=Lax; Max-Age={}",
            session_token,
            24 * 60 * 60
        );

        let clear_state_cookie = format!("{}={}; Path=/; Secure; HttpOnly; SameSite=Lax; Max-Age=0", cookie_name, "");

        if let Ok(session_header) = HeaderValue::from_str(&session_cookie) {
            response_option.append_header(SET_COOKIE, session_header);
        }

        if let Ok(clear_header) = HeaderValue::from_str(&clear_state_cookie) {
            response_option.append_header(SET_COOKIE, clear_header);
        }

        let provider_name = provider.provider_name();
        Ok(ApiResponse::data(format!("Successfully authenticated with {}", provider_name)))
    }
}
