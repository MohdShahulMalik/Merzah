#[cfg(feature = "ssr")]
use crate::auth::session::extract_bearer_token;
#[cfg(feature = "ssr")]
use crate::auth::session::get_user_by_session;
use crate::models::api_responses::ApiResponse;
#[cfg(feature = "ssr")]
use crate::models::user::User;
#[cfg(feature = "ssr")]
use actix_web::{http::StatusCode, web};
#[cfg(feature = "ssr")]
use leptos::prelude::use_context;
#[cfg(feature = "ssr")]
use leptos_actix::ResponseOptions;
#[cfg(feature = "ssr")]
use surrealdb::{Surreal, engine::remote::ws::Client};
#[cfg(feature = "ssr")]
use tracing::error;

#[cfg(feature = "ssr")]
pub async fn get_server_context<T>() -> Result<(ResponseOptions, Surreal<Client>), ApiResponse<T>> {
    let response_options = match use_context::<ResponseOptions>() {
        Some(ro) => ro,
        None => {
            error!("Failed to get ResponseOptions from context");
            return Err(ApiResponse::error("Internal Server Error".to_string()));
        }
    };

    let db = match leptos_actix::extract::<web::Data<Surreal<Client>>>().await {
        Ok(db) => db,
        Err(e) => {
            error!(?e, "Failed to extract database client");
            response_options.set_status(StatusCode::INTERNAL_SERVER_ERROR);
            return Err(ApiResponse::error("Internal Server Error".to_string()));
        }
    };

    Ok((response_options, db.get_ref().clone()))
}

/// Tells whether the current request is a direct call to a server function
/// (an API request) instead of the server side rendering of a page.
///
/// The server functions a page calls are executed inside the request of that
/// page, so the status they set is applied to the page itself. A page that is
/// publicly available must therefore not be answered with a `401 Unauthorized`
/// just because one of its internal calls needed a session, while a direct API
/// call still has to answer with one.
#[cfg(feature = "ssr")]
fn is_direct_api_request(req: &actix_web::HttpRequest) -> bool {
    let path = req.path();
    leptos::server_fn::actix::server_fn_paths()
        .any(|(server_fn_path, _)| server_fn_path == path)
}

#[cfg(feature = "ssr")]
pub async fn get_authenticated_user_and_context<T>()
-> Result<(ResponseOptions, Surreal<Client>, User), ApiResponse<T>> {
    let (response_options, db) = get_server_context::<T>().await?;

    let req = match leptos_actix::extract::<actix_web::HttpRequest>().await {
        Ok(req) => req,
        Err(e) => {
            error!(?e, "Failed to extract request");
            response_options.set_status(StatusCode::INTERNAL_SERVER_ERROR);
            return Err(ApiResponse::error("Internal Server Error".to_string()));
        }
    };

    let is_api_request = is_direct_api_request(&req);

    let session_token = if let Some(cookie) = req.cookie("__Host-session") {
        cookie.value().to_string()
    } else if let Some(auth_header) = req.headers().get("Authorization") {
        let auth_str = auth_header.to_str().unwrap_or_default();
        match extract_bearer_token(auth_str) {
            Some(token) => token,
            None => {
                if is_api_request {
                    response_options.set_status(StatusCode::UNAUTHORIZED);
                }
                return Err(ApiResponse::error("You are not logged in".to_string()));
            }
        }
    } else {
        if is_api_request {
            response_options.set_status(StatusCode::UNAUTHORIZED);
        }
        return Err(ApiResponse::error("You are not logged in".to_string()));
    };

    let user = match get_user_by_session(&session_token, &db).await {
        Ok(user) => user,
        Err(e) => {
            error!(?e, "Failed to get user by session");
            if is_api_request {
                response_options.set_status(StatusCode::UNAUTHORIZED);
            }
            return Err(ApiResponse::error("Invalid or expired session".to_string()));
        }
    };

    Ok((response_options, db, user))
}

/// Resolves the signed-in user, if there is one, without failing the request.
///
/// Unlike [`get_authenticated_user_and_context`], a missing or invalid session
/// is not treated as an error: the request still succeeds and only the data is
/// missing. A direct API call still answers with `401 Unauthorized`, while a
/// page that is publicly available keeps its own status, because the server
/// functions a page calls run inside the request of that page.
#[cfg(feature = "ssr")]
pub async fn get_optional_user_and_context<T>()
-> Result<(ResponseOptions, Surreal<Client>, Option<User>), ApiResponse<T>> {
    let (response_options, db) = get_server_context::<T>().await?;

    let req = match leptos_actix::extract::<actix_web::HttpRequest>().await {
        Ok(req) => req,
        Err(e) => {
            error!(?e, "Failed to extract request");
            return Err(ApiResponse::error("Internal Server Error".to_string()));
        }
    };

    let is_api_request = is_direct_api_request(&req);

    let session_token = if let Some(cookie) = req.cookie("__Host-session") {
        Some(cookie.value().to_string())
    } else {
        req.headers()
            .get("Authorization")
            .and_then(|auth_header| auth_header.to_str().ok())
            .and_then(extract_bearer_token)
    };

    let user = match session_token {
        Some(session_token) => match get_user_by_session(&session_token, &db).await {
            Ok(user) => Some(user),
            Err(e) => {
                error!(?e, "Failed to get user by session");
                if is_api_request {
                    response_options.set_status(StatusCode::UNAUTHORIZED);
                }
                None
            }
        },
        None => {
            if is_api_request {
                response_options.set_status(StatusCode::UNAUTHORIZED);
            }
            None
        }
    };

    Ok((response_options, db, user))
}

#[cfg(feature = "ssr")]
pub struct ServerResponse {
    options: ResponseOptions,
}

#[cfg(feature = "ssr")]
impl ServerResponse {
    pub fn new(options: ResponseOptions) -> Self {
        Self { options }
    }

    pub fn insert_header(
        &self,
        name: actix_web::http::header::HeaderName,
        value: actix_web::http::header::HeaderValue,
    ) {
        self.options.insert_header(name, value);
    }

    pub fn append_header(
        &self,
        name: actix_web::http::header::HeaderName,
        value: actix_web::http::header::HeaderValue,
    ) {
        self.options.append_header(name, value);
    }

    pub fn ok<T>(&self, data: T) -> ApiResponse<T> {
        self.options.set_status(StatusCode::OK);
        ApiResponse::data(data)
    }

    pub fn created<T>(&self, data: T) -> ApiResponse<T> {
        self.options.set_status(StatusCode::CREATED);
        ApiResponse::data(data)
    }

    pub fn accepted<T>(&self, data: T) -> ApiResponse<T> {
        self.options.set_status(StatusCode::ACCEPTED);
        ApiResponse::data(data)
    }

    pub fn no_content<T>(&self, data: T) -> ApiResponse<T> {
        self.options.set_status(StatusCode::NO_CONTENT);
        ApiResponse::data(data)
    }

    pub fn bad_request<T>(&self, error: String) -> ApiResponse<T> {
        self.options.set_status(StatusCode::BAD_REQUEST);
        ApiResponse::error(error)
    }

    pub fn unauthorized<T>(&self, error: String) -> ApiResponse<T> {
        self.options.set_status(StatusCode::UNAUTHORIZED);
        ApiResponse::error(error)
    }

    pub fn forbidden<T>(&self, error: String) -> ApiResponse<T> {
        self.options.set_status(StatusCode::FORBIDDEN);
        ApiResponse::error(error)
    }

    pub fn not_found<T>(&self, error: String) -> ApiResponse<T> {
        self.options.set_status(StatusCode::NOT_FOUND);
        ApiResponse::error(error)
    }

    pub fn method_not_allowed<T>(&self, error: String) -> ApiResponse<T> {
        self.options.set_status(StatusCode::METHOD_NOT_ALLOWED);
        ApiResponse::error(error)
    }

    pub fn unprocessable_entity<T>(&self, error: String) -> ApiResponse<T> {
        self.options.set_status(StatusCode::UNPROCESSABLE_ENTITY);
        ApiResponse::error(error)
    }

    pub fn internal_server_error<T>(&self, error: String) -> ApiResponse<T> {
        self.options.set_status(StatusCode::INTERNAL_SERVER_ERROR);
        ApiResponse::error(error)
    }

    pub fn conflict<T>(&self, error: String) -> ApiResponse<T> {
        self.options.set_status(StatusCode::CONFLICT);
        ApiResponse::error(error)
    }

    pub fn too_many_requests<T>(&self, error: String) -> ApiResponse<T> {
        self.options.set_status(StatusCode::TOO_MANY_REQUESTS);
        ApiResponse::error(error)
    }

    pub fn service_unavailable<T>(&self, error: String) -> ApiResponse<T> {
        self.options.set_status(StatusCode::SERVICE_UNAVAILABLE);
        ApiResponse::error(error)
    }
}
