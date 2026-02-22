use std::collections::HashSet;

use garde::Validate;
use leptos::{
    prelude::ServerFnError,
    server_fn::codec::{DeleteUrl, Json, PatchJson},
    *,
};

use surrealdb::RecordId;
use tracing::error;

use crate::utils::ssr::{ServerResponse, get_authenticated_user};
use crate::utils::user_elevation::is_mosque_admin;
use crate::{
    models::{
        api_responses::ApiResponse,
        events::{
            CreateEvent, Event, EventDetails, EventRecord, EventSummary, FetchedEvents,
            PersonalEvent, UpdatedEvent,
        },
    },
    utils::parsing::parse_record_id,
};

#[server(input = Json, output = Json, prefix = "/mosques/events", endpoint = "add-event")]
pub async fn add_event(create_event: CreateEvent) -> Result<ApiResponse<String>, ServerFnError> {
    let (response_options, db, user) = match get_authenticated_user::<String>().await {
        Ok(ctx) => ctx,
        Err(error) => return Ok(error),
    };
    let responder = ServerResponse::new(response_options);

    let validation_result = create_event.validate();
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

    let event_record = EventRecord::from(create_event);

    let create_event_transaction = r#"
        BEGIN TRANSACTION;
        LET $event = (CREATE ONLY events CONTENT $event_data);
        RELATE ($event.mosque) -> hosts -> $event SET created_by = $user_id;
        COMMIT TRANSACTION;
    "#;

    let transaction_result = db
        .query(create_event_transaction)
        .bind(("event_data", event_record))
        .bind(("user_id", user.id))
        .await;

    match transaction_result {
        Ok(result) => {
            if let Err(err) = result.check() {
                return Ok(responder.internal_server_error(format!(
                    "Some db error occured during the transaction: {err}"
                )));
            }
        }

        Err(err) => {
            return Ok(responder.internal_server_error(format!(
                "Some db error occured while executing the transaction: {err}"
            )));
        }
    }

    Ok(responder.created("Successfully created the event record Alhadulillah!".to_string()))
}

#[server(input = PatchJson, output = Json, prefix = "/mosques/events", endpoint = "/update-event")]
pub async fn update_event(
    event_id: String,
    updated_event: UpdatedEvent,
) -> Result<ApiResponse<String>, ServerFnError> {
    let (response_options, db, _user) = match get_authenticated_user::<String>().await {
        Ok(ctx) => ctx,
        Err(err) => return Ok(err),
    };

    let responder = ServerResponse::new(response_options);

    let event_id: RecordId = match parse_record_id(&event_id, "event_id") {
        Ok(id) => id,
        Err(e) => return Ok(e),
    };

    let validation_result = updated_event.validate();
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

    let update_event_transaction = r#"
        BEGIN TRANSACTION;
        LET $event = (UPDATE ONLY $event_id MERGE $updated_event);
        IF $event != NONE {
            UPDATE hosts SET updated_at = time::now() WHERE out = $event_id;
        };
        COMMIT TRANSACTION;
        RETURN $event;
    "#;

    let transaction_result = db
        .query(update_event_transaction)
        .bind(("event_id", event_id))
        .bind(("updated_event", updated_event))
        .await;

    match transaction_result {
        Ok(result) => {
            let mut result = match result.check() {
                Ok(r) => r,
                Err(err) => {
                    return Ok(responder.internal_server_error(format!(
                        "Some db error occured during the transaction: {err}"
                    )));
                }
            };

            let event: Option<Event> = match result.take(2) {
                Ok(event) => event,
                Err(err) => {
                    return Ok(responder.internal_server_error(format!(
                        "Some db error occured while fetching the updated event: {err}"
                    )));
                }
            };

            if event.is_none() {
                return Ok(responder.not_found("No event found with the provided ID".to_string()));
            }
        }

        Err(err) => {
            return Ok(responder.internal_server_error(format!(
                "Some db error occured while executing the transaction: {err}"
            )));
        }
    }

    Ok(responder.ok("successfully updated the event record".to_string()))
}

#[server(input = Json, output = Json, prefix = "/mosques/events", endpoint = "/fetch-users-favorite-mosques-events")]
pub async fn fetch_users_favorite_mosques_events()
-> Result<ApiResponse<Vec<PersonalEvent>>, ServerFnError> {
    let (response_options, db, user) = match get_authenticated_user::<Vec<PersonalEvent>>().await {
        Ok(ctx) => ctx,
        Err(err) => return Ok(err),
    };
    let responder = ServerResponse::new(response_options);

    let events_and_rsvp_query = r#"
        $user_id ->favorited->mosques->hosts->events.*;
        $user_id -> attending -> events;
    "#;

    let events_and_rsvp_query_result = db
        .query(events_and_rsvp_query)
        .bind(("user_id", user.id.clone()))
        .await;

    let mut db_response = match events_and_rsvp_query_result {
        Ok(response) => response,
        Err(err) => {
            return Ok(responder.internal_server_error(format!("Some db error occured: {err}")));
        }
    };

    let events = match db_response.take::<Vec<EventDetails>>(0) {
        Ok(events) => events,
        Err(err) => {
            return Ok(responder.internal_server_error(format!("Some db error occured: {err}")));
        }
    };

    let rsvp = match db_response.take::<Vec<String>>(1) {
        Ok(rsvp_result) => rsvp_result,
        Err(err) => {
            return Ok(responder.internal_server_error(format!("Some db error occured: {err}")));
        }
    };

    let rsvp_set: HashSet<String> = rsvp.into_iter().collect();

    let personal_events: Vec<PersonalEvent> = events
        .into_iter()
        .map(|event| {
            let is_attending = rsvp_set.contains(&event.id);
            PersonalEvent::new(event, is_attending)
        })
        .collect();

    Ok(responder.ok(personal_events))
}

#[server(input = Json, output = Json, prefix = "/mosques/events", endpoint = "/fetch-mosque-events")]
pub async fn fetch_mosque_events(
    mosque_id: String,
) -> Result<ApiResponse<FetchedEvents>, ServerFnError> {
    let (response_options, db, user) = match get_authenticated_user::<FetchedEvents>().await {
        Ok(ctx) => ctx,
        Err(e) => return Ok(e),
    };

    let responder = ServerResponse::new(response_options);

    let mosque_id: RecordId = match parse_record_id(&mosque_id, "mosque_id") {
        Ok(id) => id,
        Err(e) => return Ok(e),
    };

    let is_admin = is_mosque_admin(&user.id, &mosque_id, &db).await.is_ok();

    if is_admin {
        let query = r#"
            SELECT 
                {
                    id: type::string(id),
                    title: title,
                    description: description,
                    category: category,
                    date: date,
                    speaker: speaker
                } AS event,

                array::len(<-attending)
                AS rsvp_count

            FROM $mosque_id->hosts->events
        "#;

        let query_result = db.query(query).bind(("mosque_id", mosque_id)).await;

        let events: Vec<EventSummary> = match query_result {
            Ok(mut response) => response.take(0).unwrap_or_default(),
            Err(err) => {
                return Ok(responder.internal_server_error(format!("Some db error occured: {err}")));
            }
        };

        Ok(responder.ok(FetchedEvents::Summary(events)))
    } else {
        let query = r#"
            SELECT 
                {
                    id: type::string(id),
                    title: title,
                    description: description,
                    category: category,
                    date: date,
                    speaker: speaker
                } AS event,

                (array::len(<-attending WHERE in = $user_id) == 1)
                AS rsvp

            FROM $mosque_id->hosts->events
        "#;

        let query_result = db
            .query(query)
            .bind(("mosque_id", mosque_id))
            .bind(("user_id", user.id))
            .await;

        let events: Vec<PersonalEvent> = match query_result {
            Ok(mut response) => response.take(0).unwrap_or_default(),
            Err(err) => {
                return Ok(responder.internal_server_error(format!("Some db error occured: {err}")));
            }
        };

        Ok(responder.ok(FetchedEvents::Personal(events)))
    }
}

#[server(input = DeleteUrl, output = Json, prefix = "/mosques/events", endpoint = "/delete/")]
pub async fn delete_event(event_id: String) -> Result<ApiResponse<String>, ServerFnError> {
    tracing::info!(?event_id, "delete_event called with event_id");

    let (response_options, db, _user) = match get_authenticated_user::<String>().await {
        Ok(ctx) => ctx,
        Err(err) => return Ok(err),
    };

    let responder = ServerResponse::new(response_options);

    let event_id: RecordId = match parse_record_id(&event_id, "event_id") {
        Ok(id) => id,
        Err(e) => return Ok(e),
    };

    let delete_event_transaction = r#"
        BEGIN TRANSACTION;
        DELETE hosts WHERE out = $event_id;
        DELETE attending WHERE out = $event_id;
        LET $deleted = (DELETE ONLY $event_id RETURN BEFORE);
        COMMIT TRANSACTION;
        RETURN $deleted;
    "#;

    let transaction_result = db
        .query(delete_event_transaction)
        .bind(("event_id", event_id))
        .await;

    match transaction_result {
        Ok(result) => {
            let mut result = match result.check() {
                Ok(r) => r,
                Err(err) => {
                    return Ok(responder.internal_server_error(format!(
                        "Some db error occured during the transaction: {err}"
                    )));
                }
            };

            let event: Option<Event> = match result.take(3) {
                Ok(event) => event,
                Err(err) => {
                    return Ok(responder.internal_server_error(format!(
                        "Some db error occured while fetching the deleted event: {err}"
                    )));
                }
            };

            if event.is_none() {
                return Ok(responder.not_found("No event found with the provided ID".to_string()));
            }
        }

        Err(err) => {
            return Ok(responder.internal_server_error(format!(
                "Some db error occured while executing the transaction: {err}"
            )));
        }
    }

    Ok(responder.ok("Successfully deleted the event record".to_string()))
}
