use garde::Validate;
use leptos::prelude::ServerFnError;
use leptos::{ *, server_fn::codec::{ Json } };

use tracing::error;

use crate::models::{api_responses::ApiResponse, events::CreateEvent};

// TODO: complete this server_fn
#[server(input = Json, output = Json, prefix = "/mosques/events", endpoint = "add-event")]
pub async fn add_event(event: CreateEvent) -> Result<ApiResponse<String>, ServerFnError> {
    let validation_result = event.validate();
    if let Err(err) = validation_result {
        let errors = err.iter()
            .map(|(field, msg)| format!("{field}: {msg}"))
            .collect::<Vec<_>>();
        error!(?errors);
    }

    Ok(ApiResponse::data("Created the event successfully".to_string()))
}
