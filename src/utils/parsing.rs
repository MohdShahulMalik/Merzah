use crate::models::api_responses::ApiResponse;

#[cfg(feature = "ssr")]
use surrealdb::RecordId;
#[cfg(feature = "ssr")]
use leptos_actix::ResponseOptions;
#[cfg(feature = "ssr")]
use actix_web::http::StatusCode;
#[cfg(feature = "ssr")]
use leptos::prelude::expect_context;

#[cfg(feature = "ssr")]
pub fn parse_record_id(id: &str, field_name: &str) -> Result<RecordId, ApiResponse<String>> {
    id.parse().map_err(|e| {
        tracing::error!(?e, "Failed to parse {}", field_name);
        
        let response_options = expect_context::<ResponseOptions>();
        response_options.set_status(StatusCode::BAD_REQUEST);
        

        ApiResponse::error(format!("Failed to parse {}", field_name))
    })
}
