use leptos::prelude::ServerFnError;
use leptos::server_fn::codec::Json;
use leptos::*;
use crate::models::auth::LoginFormData;
use crate::models::{api_responses::ApiResponse, auth::RegistrationFormData};

#[server(input=Json, prefix = "/auth", endpoint = "register")]
pub async fn register(form: RegistrationFormData) -> Result<ApiResponse<String>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use actix_web::{http::StatusCode, web};
        use garde::Validate;
        use leptos::prelude::expect_context;
        use leptos_actix::ResponseOptions;
        use tracing::error;
        use surrealdb::Surreal;
        use surrealdb::engine::remote::ws::Client;
        use crate::auth::custom_auth::register_user;
        use crate::auth::session::{create_session, set_session_cookie};

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
    #[cfg(not(feature = "ssr"))]
    {
        let _ = form;
        unreachable!()
    }
}

#[server(input=Json, prefix="/auth", endpoint="login")]
pub async fn login(
    form: LoginFormData,
) -> Result<ApiResponse<String>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use actix_web::{http::StatusCode, web};
        use leptos::prelude::expect_context;
        use leptos_actix::ResponseOptions;
        use tracing::error;
        use surrealdb::Surreal;
        use surrealdb::engine::remote::ws::Client;
        use crate::auth::custom_auth::authenticate;
        use crate::auth::session::{create_session, set_session_cookie};
        use crate::errors::auth::AuthError;

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
}

#[server(input=Json, prefix="/auth", endpoint="login")]
pub async fn logout() -> Result<ApiResponse<String>, ServerFnError> {
    
}
