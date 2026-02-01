use garde::Validate;
use leptos::{ prelude::ServerFnError, server_fn::codec::{Json, PatchJson}, * };

use surrealdb::RecordId;
use tracing::error;

use crate::{models::{api_responses::ApiResponse, events::{ CreateEvent, Event, UpdatedEvent }}, utils::parsing::parse_record_id};
use crate::utils::ssr::{ServerResponse, get_server_context};

#[server(input = Json, output = Json, prefix = "/mosques/events", endpoint = "add-event")]
pub async fn add_event(event: CreateEvent) -> Result<ApiResponse<String>, ServerFnError> {
    let (response_options, db) = match get_server_context().await {
        Ok(ctx) => ctx,
        Err(error) => return Ok(error),
    };
    let responder = ServerResponse::new(response_options);

    let validation_result = event.validate();
    if let Err(err) = validation_result {
        let errors = err
            .iter()
            .map(|(field, msg)| format!("{field}: {msg}"))
            .collect::<Vec<_>>();
        error!(?errors);
        let error =
            responder.unprocessable_entity("Error while validating the event's data".to_string());
        return Ok(error);
    }

    let create_result: Result<Option<Event>, surrealdb::Error> =
        db.create("events").content(event).await;

    match create_result {
        Ok(Some(_)) => (),
        Ok(None) => {
            return Ok(responder.not_found(
                "Some db error occured: query successfully executed but no record was created"
                    .to_string(),
            ));
        }
        Err(err) => {
            return Ok(responder.internal_server_error(format!("Some db error occured: {err}")));
        }
    };

    Ok(responder.created("Successfully created the event record Alhadulillah!".to_string()))
}

#[server(input = PatchJson, output = Json, prefix = "/mosques/events", endpoint = "/update-event")]
pub async fn update_event(event_id: String, updated_event: UpdatedEvent) -> Result<ApiResponse<String>, ServerFnError> {
    let (response_options, db) = match get_server_context().await {
        Ok(ctx) => ctx,
        Err(err) => return Ok(err),
    };

    let event_id: RecordId = match parse_record_id(&event_id, "event_id") {
        Ok(id) => id,
        Err(e) => return Ok(e),
    };
    let responder = ServerResponse::new(response_options);

    let update_result = db.update::<Option<Event>>(event_id)
        .merge(updated_event)
        .await;

    match update_result {
        Ok(Some(_)) => (),
        Ok(None) => {
            return Ok(responder.not_found("No event found with the provided ID".to_string()));
        },
        Err(err) => {
            return Ok(responder.internal_server_error(format!("Some db error occured: {err}")));
        }
    }
    
    Ok(responder.ok("successfully updated the event record".to_string()))
}
