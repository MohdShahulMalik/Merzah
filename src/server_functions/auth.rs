use leptos::prelude::ServerFnError;
use leptos::server_fn::codec::{DeleteUrl, Json};
use leptos::*; 
use crate::models::auth::LoginFormData;
use crate::models::{api_responses::ApiResponse, auth::{RegistrationFormData, Platform}};
#[cfg(feature = "ssr")]
use garde::Validate;

#[cfg(feature = "ssr")]
use actix_web::{http::StatusCode, HttpRequest};
#[cfg(feature = "ssr")]
use crate::utils::ssr::get_server_context;
#[cfg(feature = "ssr")]
use tracing::error;
#[cfg(feature = "ssr")]
use crate::auth::custom_auth::{authenticate, register_user};
#[cfg(feature = "ssr")]
use crate::auth::session::{create_session, delete_session, remove_session_cookie, set_session_cookie};
#[cfg(feature = "ssr")]
use crate::errors::session::SessionError;
#[cfg(feature = "ssr")]
use crate::errors::auth::AuthError;

#[server(input=Json, prefix = "/auth", endpoint = "register")]
pub async fn register(form: RegistrationFormData) -> Result<ApiResponse<String>, ServerFnError> {
    let (response_option, db) = match get_server_context().await {
        Ok(ctx) => ctx,
        Err(e) => return Ok(e),
    };

    let validation_result = form.validate();

    if let Err(error) = validation_result {
        let errors = error
            .iter()
            .map(|(field, msg)| format!("{}, {}", field, msg))
            .collect::<Vec<_>>();
        error!(?errors);
        response_option.set_status(StatusCode::UNPROCESSABLE_ENTITY);
        return Ok(ApiResponse { data: None, error: Some(errors.join("\n"))})
    }

    let validation_result_for_uniqueness = form.validate_uniqueness(&db).await;
    if let Err(error) = validation_result_for_uniqueness {
        error!(?error);
        response_option.set_status(StatusCode::CONFLICT);
        return Ok(ApiResponse { data: None, error: Some(format!("{}", error))});
    } 

    let registration_result = register_user(form.clone(), &db).await;

    if let Err(error) = registration_result {
        error!(?error, "Failed to register the user");  
        return Err(ServerFnError::ServerError("Failed to register the user".to_string()));
    };

    let user_id = registration_result.ok();
    let session_creation_result = create_session(user_id.unwrap(), &db).await;
    if let Err(error) = session_creation_result {
        error!(?error);
        return Err(ServerFnError::ServerError("Failed to generate session tokens for the registered user".to_string()));
    }

    let session_token = session_creation_result.ok().unwrap();
    
    if let Platform::Web = form.platform {
        let cookie_creation_result = set_session_cookie(&session_token);

        if let Err(error) = cookie_creation_result {
            error!(?error);
            return Err(ServerFnError::ServerError("Failed to create appropriate cookies after registration".to_string()));
        }

        Ok(ApiResponse {
            data: Some("The user has been registered successfully".to_string()),
            error: None,
        })
    } else {
        Ok(ApiResponse {
            data: Some(session_token),
            error: None,
        })
    }
}

#[server(input=Json, prefix="/auth", endpoint="login")]
pub async fn login(
    form: LoginFormData,
) -> Result<ApiResponse<String>, ServerFnError> {
    let (response_option, db) = match get_server_context().await {
        Ok(ctx) => ctx,
        Err(e) => return Ok(e),
    };

    let user_id = match authenticate(form.clone(), &db).await {
        Ok(id) => id,
        Err(error) => {
            if let Some(auth_error) = error.downcast_ref::<AuthError>() {
                match auth_error {
                    AuthError::UserNotFound | AuthError::PasswordVerificationError(_) => {
                        error!("Authentication failed for user.");
                        response_option.set_status(StatusCode::UNAUTHORIZED);
                        return Ok(ApiResponse { data: None, error: Some("Invalid username or password.".to_string())});
                    },
                    AuthError::DatabaseError(_) | AuthError::PasswordHashError(_) => {
                        error!(?error, "Internal server error during authentication.");
                        response_option.set_status(StatusCode::INTERNAL_SERVER_ERROR);
                        return Ok(ApiResponse { data: None, error: Some("An internal error occurred.".to_string())});
                    },
                    _ => {
                        error!(?error, "An unexpected authentication error occurred.");
                        response_option.set_status(StatusCode::INTERNAL_SERVER_ERROR);
                        return Ok(ApiResponse { data: None, error: Some("An internal error occurred.".to_string())});
                    }
                }
            } else {
                error!(?error, "An unexpected error occurred during login.");
                response_option.set_status(StatusCode::INTERNAL_SERVER_ERROR);
                return Ok(ApiResponse { data: None, error: Some("An internal error occurred.".to_string())});
            }
        }
    };

    let session_creation_result = create_session(user_id, &db).await;
    if let Err(error) = session_creation_result {
        error!(?error);
        response_option.set_status(StatusCode::INTERNAL_SERVER_ERROR);
        return Ok(ApiResponse { data: None, error: Some("Failed to create user session.".to_string())});
    }

    let session_token = session_creation_result.ok().unwrap();
    
    if let Platform::Web = form.platform {
        let cookie_creation_result = set_session_cookie(&session_token);

        if let Err(error) = cookie_creation_result {
            error!(?error);
            response_option.set_status(StatusCode::INTERNAL_SERVER_ERROR);
            return Ok(ApiResponse { data: None, error: Some("Failed to set session cookie.".to_string())});
        }

        Ok(ApiResponse {
            data: Some("The user has been logged in successfully".to_string()),
            error: None,
        })
    } else {
        Ok(ApiResponse {
            data: Some(session_token),
            error: None,
        })
    }
}

#[server(input=DeleteUrl, output=Json, prefix="/auth", endpoint="logout")]
pub async fn logout() -> Result<ApiResponse<String>, ServerFnError> {
    let (response_option, db) = match get_server_context().await {
        Ok(ctx) => ctx,
        Err(e) => return Ok(e),
    };

    let req = match leptos_actix::extract::<HttpRequest>().await {
        Ok(req) => req,
        Err(e) => {
            error!(?e, "Failed to extract request");
            response_option.set_status(StatusCode::INTERNAL_SERVER_ERROR);
            return Ok(ApiResponse::error("Internal server error".to_string()));
        }
    };

    let session_token = if let Some(cookie) = req.cookie("__Host-session") {
        cookie.value().to_string()
    } else if let Some(auth_header) = req.headers().get("Authorization") {
        let auth_str = auth_header.to_str().unwrap_or("");
        if auth_str.starts_with("Bearer ") {
            auth_str.trim_start_matches("Bearer ").to_string()
        } else {
            response_option.set_status(StatusCode::UNAUTHORIZED);
            return Ok(ApiResponse::error("You are not logged in".to_string()));
        }
    } else {
        response_option.set_status(StatusCode::UNAUTHORIZED);
        return Ok(ApiResponse::error("You are not logged in".to_string()));
    };

    if let Err(e) = delete_session(&session_token, &db).await {
        error!(?e, "Failed to delete session");
        let session_error_option = e.downcast_ref::<SessionError>();
        if let Some(session_error) = session_error_option {
            match session_error {
                SessionError::SessionExpired(_) => {
                    response_option.set_status(StatusCode::UNAUTHORIZED);
                    return Ok(ApiResponse::error("Your session has expired".to_string()));
                }
                SessionError::SessionNotFound => {
                    response_option.set_status(StatusCode::UNAUTHORIZED);
                    return Ok(ApiResponse::error("Session not found".to_string()));
                }
                SessionError::InvalidToken => {
                    response_option.set_status(StatusCode::UNAUTHORIZED);
                    return Ok(ApiResponse::error("Invalid session token".to_string()));
                }
                SessionError::UserNotFound => {
                    response_option.set_status(StatusCode::UNAUTHORIZED);
                    return Ok(ApiResponse::error("User not found for this session".to_string()));
                }
                SessionError::DatabaseError(err) => {
                    response_option.set_status(StatusCode::INTERNAL_SERVER_ERROR);
                    return Ok(ApiResponse::error(format!("Database error occurred: {}", err)));
                }
            }
        }
        response_option.set_status(StatusCode::INTERNAL_SERVER_ERROR);
        return Ok(ApiResponse::error("Failed to delete the session from the server".to_string()));
    }

    // Only attempt to remove cookie if it was present
    if req.cookie("__Host-session").is_some() {
        if let Err(e) = remove_session_cookie() {
            error!(?e, "Failed to remove session cookie");
            response_option.set_status(StatusCode::INTERNAL_SERVER_ERROR);
            return Ok(ApiResponse::error("Failed to remove session cookie".to_string()));
        }
    }

    Ok(ApiResponse::data("Successfully logged out the user".to_string()))
}
