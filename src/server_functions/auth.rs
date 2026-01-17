use leptos::prelude::ServerFnError;
use leptos::server_fn::codec::{DeleteUrl, Json};
use leptos::*; 
use crate::models::auth::LoginFormData;
use crate::models::{api_responses::ApiResponse, auth::RegistrationFormData};
#[cfg(feature = "ssr")]
use garde::Validate;

#[cfg(feature = "ssr")]
use actix_web::{http::StatusCode, web, HttpRequest};
#[cfg(feature = "ssr")]
use leptos::prelude::expect_context;
#[cfg(feature = "ssr")]
use leptos_actix::ResponseOptions;
#[cfg(feature = "ssr")]
use surrealdb::{Surreal, engine::remote::ws::Client};
#[cfg(feature = "ssr")]
use tracing::error;
#[cfg(feature = "ssr")]
use crate::auth::custom_auth::{authenticate, register_user};
#[cfg(feature = "ssr")]
use crate::auth::session::{create_session, delete_session, remove_session_cookie, set_session_cookie};
#[cfg(feature = "ssr")]
use crate::errors::auth::AuthError;

#[server(input=Json, prefix = "/auth", endpoint = "register")]
pub async fn register(form: RegistrationFormData) -> Result<ApiResponse<String>, ServerFnError> {
    let response_option = expect_context::<ResponseOptions>();
    let db = leptos_actix::extract::<web::Data<Surreal<Client>>>().await?;

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

    let registration_result = register_user(form, &db).await;

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
    let cookie_creation_result = set_session_cookie(&session_token);

    if let Err(error) = cookie_creation_result {
        error!(?error);
        return Err(ServerFnError::ServerError("Failed to create appropriate cookies after registration".to_string()));
    }

    Ok(ApiResponse {
        data: Some("The user have been registered successfully".to_string()),
        error: None,
    })
}

#[server(input=Json, prefix="/auth", endpoint="login")]
pub async fn login(
    form: LoginFormData,
) -> Result<ApiResponse<String>, ServerFnError> {
    let response_option = expect_context::<ResponseOptions>();
    let db = leptos_actix::extract::<web::Data<Surreal<Client>>>().await?;

    let user_id = match authenticate(form, &db).await {
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
}

#[server(input=DeleteUrl, output=Json, prefix="/auth", endpoint="logout")]
pub async fn logout() -> Result<ApiResponse<String>, ServerFnError> {
    let response_option = expect_context::<ResponseOptions>();

    let req = match leptos_actix::extract::<HttpRequest>().await {
        Ok(req) => req,
        Err(e) => {
            error!(?e, "Failed to extract request");
            response_option.set_status(StatusCode::INTERNAL_SERVER_ERROR);
            return Ok(ApiResponse::error("Internal server error".to_string()));
        }
    };

    let session_token = match req.cookie("__Host-session") {
        Some(cookie) => cookie.value().to_string(),
        None => {
            response_option.set_status(StatusCode::UNAUTHORIZED);
            return Ok(ApiResponse::error("You are not logged in".to_string()));
        }
    };

    let db = match leptos_actix::extract::<web::Data<Surreal<Client>>>().await {
        Ok(db) => db,
        Err(e) => {
            error!(?e, "Failed to extract database connection");
            response_option.set_status(StatusCode::INTERNAL_SERVER_ERROR);
            return Ok(ApiResponse::error("Internal server error due to not getting db connection".to_string()));
        }
    };

    if let Err(e) = delete_session(&session_token, &db).await {
        error!(?e, "Failed to delete session");
        response_option.set_status(StatusCode::INTERNAL_SERVER_ERROR);
        return Ok(ApiResponse::error("Failed to delete the session cokkie from the server".to_string()));
    }

    if let Err(e) = remove_session_cookie() {
        error!(?e, "Failed to remove session cookie");
        response_option.set_status(StatusCode::INTERNAL_SERVER_ERROR);
        return Ok(ApiResponse::error("Failed to remove session cookie".to_string()));
    }

    Ok(ApiResponse::data("Successfully logged out the user".to_string()))
}