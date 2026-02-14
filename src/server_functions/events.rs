use std::collections::HashSet;

use garde::Validate;
use leptos::{ prelude::ServerFnError, server_fn::codec::{Json, PatchJson}, * };

use surrealdb::RecordId;
use tracing::error;

use crate::{models::{api_responses::ApiResponse, events::{ CreateEvent, Event, EventSummary, FetchedEvents, PersonalEvent, UpdatedEvent }}, utils::{parsing::parse_record_id, ssr::get_server_context}};
use crate::utils::ssr::{ServerResponse, get_authenticated_user};
use crate::utils::user_elevation::is_mosque_admin;

#[server(input = Json, output = Json, prefix = "/mosques/events", endpoint = "add-event")]
pub async fn add_event(mosque_id: String, event: CreateEvent) -> Result<ApiResponse<String>, ServerFnError> {
    let (response_options, db, user) = match get_authenticated_user::<String>().await {
        Ok(ctx) => ctx,
        Err(error) => return Ok(error),
    };
    let responder = ServerResponse::new(response_options);

    let mosque_id: RecordId = match parse_record_id(&mosque_id, "mosque_id") {
        Ok(id) => id,
        Err(e) => return Ok(e),
    };

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

    let event: Event = match create_result {
        Ok(Some(ev)) => ev,
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

    let host_query = r#"RELATE $mosque_id -> hosts -> $event_id
        SET created_by = $user_id
    "#;

    let host_result = db.query(host_query)
        .bind(("mosque_id", mosque_id))
        .bind(("event_id", event.id.clone()))
        .bind(("user_id", user.id))
        .await;

    match host_result {
        Ok(_) => (),
        Err(err) => {
            return Ok(responder.internal_server_error(format!("Some db error occured while creating the relationship between the event and the mosque: {err}")));
        },
    }

    Ok(responder.created("Successfully created the event record Alhadulillah!".to_string()))
}

#[server(input = PatchJson, output = Json, prefix = "/mosques/events", endpoint = "/update-event")]
pub async fn update_event(event_id: String, updated_event: UpdatedEvent) -> Result<ApiResponse<String>, ServerFnError> {
    let (response_options, db, _user) = match get_authenticated_user::<String>().await {
        Ok(ctx) => ctx,
        Err(err) => return Ok(err),
    };

    let event_id: RecordId = match parse_record_id(&event_id, "event_id") {
        Ok(id) => id,
        Err(e) => return Ok(e),
    };
    let responder = ServerResponse::new(response_options);

    let update_result = db.update::<Option<Event>>(event_id.clone())
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

    let update_hosts_query = r#"
        UPDATE hosts
        SET updated_at = time::now()
        WHERE out = $event_id;
    "#;

    let update_hosts_result = db.query(update_hosts_query)
        .bind(("event_id", event_id.clone()))
        .await;

    match update_hosts_result {
        Ok(_) => (),
        Err(err) => {
            return Ok(responder.internal_server_error(format!("Some db error occured while updating the related hosts records: {err}")));
        },
    }
    
    Ok(responder.ok("successfully updated the event record".to_string()))
}

#[server(input = Json, output = Json, prefix = "/mosques/events", endpoint = "/fetch-users-favorite-mosques-events")]
pub async fn fetch_users_favorite_mosques_events() -> Result<ApiResponse<Vec<PersonalEvent>>, ServerFnError> {
    let (response_options, db, user) = match get_authenticated_user::<Vec<PersonalEvent>>().await {
        Ok(ctx) => ctx,
        Err(err) => return Ok(err),
    };
    let responder = ServerResponse::new(response_options);

    let events_and_rsvp_query = r#"
        $user_id ->favorited->mosques->hosts->events.*;
        $user_id -> attending -> events;
    "#;

    let events_and_rsvp_query_result = db.query(events_and_rsvp_query)
        .bind(("user_id", user.id.clone()))
        .await;

    let mut db_response =  match events_and_rsvp_query_result {
        Ok(response) => response,
        Err(err) => return Ok(responder.internal_server_error(format!("Some db error occured: {err}"))),
    };

    let events = match db_response.take::<Vec<Event>>(0) {
        Ok(events) => events,
        Err(err) => return Ok(responder.internal_server_error(format!("Some db error occured: {err}"))),
    };

    let rsvp = match db_response.take::<Vec<String>>(1) {
        Ok(rsvp_result) => rsvp_result,
        Err(err) => return Ok(responder.internal_server_error(format!("Some db error occured: {err}"))),
    };

    let rsvp_set: HashSet<String> = rsvp.into_iter().collect();

    let personal_events: Vec<PersonalEvent> = events.into_iter().map(|event| {
        let is_attending = rsvp_set.contains(&event.id);
        PersonalEvent::new(event, is_attending)
    }).collect();

    Ok(responder.ok(personal_events))
}

#[server(input = Json, output = Json, prefix = "/mosques/events", endpoint = "/fetch-mosque-events")]
pub async fn fetch_mosque_events(mosque_id: String) -> Result<ApiResponse<FetchedEvents>, ServerFnError> {
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
                { id: id, title: title, description: description, category: category, date: date, speaker: speaker } AS event,
                count(<-attending) AS rsvp_count
            FROM events
            WHERE id IN $mosque_id->hosts->events
            GROUP BY ALL
        "#;

        let query_result = db.query(query)
            .bind(("mosque_id", mosque_id))
            .await;

        let events: Vec<EventSummary> = match query_result {
            Ok(mut response) => response.take(0).unwrap_or_default(),
            Err(err) => return Ok(responder.internal_server_error(format!("Some db error occured: {err}"))),
        };

        Ok(responder.ok(FetchedEvents::Summary(events)))
    } else {
        let query = r#"
            SELECT 
                { id: id, title: title, description: description, category: category, date: date, speaker: speaker } AS event,
                (count(<-attending WHERE in = $user_id) == 1) AS rsvp
            FROM events
            WHERE id IN $mosque_id->hosts->events
            GROUP BY ALL
        "#;

        let query_result = db.query(query)
            .bind(("mosque_id", mosque_id))
            .bind(("user_id", user.id))
            .await;

        let events: Vec<PersonalEvent> = match query_result {
            Ok(mut response) => response.take(0).unwrap_or_default(),
            Err(err) => return Ok(responder.internal_server_error(format!("Some db error occured: {err}"))),
        };

        Ok(responder.ok(FetchedEvents::Personal(events)))
    }
}

