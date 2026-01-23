use crate::models::api_responses::ApiResponse;
#[cfg(feature = "ssr")]
use leptos::prelude::use_context;
#[cfg(feature = "ssr")]
use leptos_actix::ResponseOptions;
#[cfg(feature = "ssr")]
use surrealdb::{Surreal, engine::remote::ws::Client};
#[cfg(feature = "ssr")]
use actix_web::{web, http::StatusCode};
#[cfg(feature = "ssr")]
use tracing::error;

#[cfg(feature = "ssr")]
pub async fn get_server_context() -> Result<(ResponseOptions, Surreal<Client>), ApiResponse<String>> {
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