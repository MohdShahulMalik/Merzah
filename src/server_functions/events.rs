use garde::Validate;
use leptos::{ server_fn::codec::Json, prelude::ServerFnError, * };

use tracing::error;

use crate::models::{api_responses::ApiResponse, events::{ CreateEvent, Event }};
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
            return Ok(responder.internal_server_error(
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
