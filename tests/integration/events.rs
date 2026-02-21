use crate::common::get_test_db;
use chrono::{Duration, FixedOffset, Utc};
use merzah::{
    auth::session::create_session,
    models::{
        api_responses::ApiResponse,
        events::{CreateEvent, Event, EventCategory, EventRecurrence, Interval, UpdatedEvent},
        mosque::MosqueRecord,
        user::User,
    },
    services::recurrence::{calculate_next_date, check_and_rotate_events},
    spawn_app,
};
use reqwest::Client;
use rstest::rstest;
use serde::Serialize;
use surrealdb::{Datetime, RecordId, sql::Geometry};

#[derive(Serialize)]
struct CreateMosque {
    pub location: Geometry,
    pub name: String,
}

#[derive(Serialize)]
struct AddEventParams {
    pub mosque_id: String,
    pub create_event: CreateEvent,
}

#[derive(Serialize)]
struct UpdateEventParams {
    pub event_id: String,
    pub updated_event: UpdatedEvent,
}

#[derive(Serialize)]
struct RsvpParams {
    pub event_id: String,
}

#[derive(Debug, Clone, Copy)]
enum AuthMethod {
    Web,
    Mobile,
}

fn build_auth_headers(client: &Client, session: &str, auth_method: AuthMethod, url: &str) -> reqwest::RequestBuilder {
    match auth_method {
        AuthMethod::Web => client.post(url).header("Cookie", format!("__Host-session={}", session)),
        AuthMethod::Mobile => client.post(url).header("Authorization", format!("Bearer {}", session)),
    }
}

fn build_auth_patch(client: &Client, session: &str, auth_method: AuthMethod, url: &str) -> reqwest::RequestBuilder {
    match auth_method {
        AuthMethod::Web => client.patch(url).header("Cookie", format!("__Host-session={}", session)),
        AuthMethod::Mobile => client.patch(url).header("Authorization", format!("Bearer {}", session)),
    }
}

fn build_auth_delete(client: &Client, session: &str, auth_method: AuthMethod, url: &str) -> reqwest::RequestBuilder {
    match auth_method {
        AuthMethod::Web => client.delete(url).header("Cookie", format!("__Host-session={}", session)),
        AuthMethod::Mobile => client.delete(url).header("Authorization", format!("Bearer {}", session)),
    }
}

async fn setup_user_and_session(db: &surrealdb::Surreal<surrealdb::engine::remote::ws::Client>) -> (User, String) {
    let user_id = RecordId::from(("users", format!("user_{}", uuid::Uuid::new_v4())));
    let user: User = db
        .create(user_id.clone())
        .content(User {
            id: user_id.clone(),
            created_at: Datetime::default(),
            display_name: "Test User".to_string(),
            password_hash: "hash".to_string(),
            role: "regular".to_string(),
            updated_at: Datetime::default(),
        })
        .await
        .expect("Failed to create user")
        .expect("Not returned");

    let session = create_session(user.id.clone(), db).await.expect("Failed to create session");
    (user, session)
}

async fn setup_mosque(db: &surrealdb::Surreal<surrealdb::engine::remote::ws::Client>) -> MosqueRecord {
    db.create("mosques")
        .content(CreateMosque {
            location: Geometry::Point((0.0, 0.0).into()),
            name: "Test Mosque".to_string(),
        })
        .await
        .expect("Failed to create mosque")
        .expect("Not returned")
}

async fn create_event_via_api(
    client: &Client,
    addr: &str,
    session: &str,
    auth_method: AuthMethod,
    mosque_id: &str,
    event: CreateEvent,
) -> ApiResponse<String> {
    let url = format!("{}/mosques/events/add-event", addr);
    let params = AddEventParams {
        mosque_id: mosque_id.to_string(),
        create_event: event,
    };

    let req = build_auth_headers(client, session, auth_method, &url);
    let response = req.json(&params).send().await.expect("Failed to send request");

    assert!(response.status().is_success(), "Failed to create event: {:?}", response.text().await);
    response.json().await.expect("Failed to deserialize response")
}

#[rstest]
#[case::web(AuthMethod::Web)]
#[case::mobile(AuthMethod::Mobile)]
#[tokio::test]
async fn test_create_recurring_event_via_api(
    #[case] auth_method: AuthMethod,
) {
    let db = get_test_db().await;
    let addr = spawn_app(db.clone());
    let client = Client::new();

    let (_user, session) = setup_user_and_session(&db).await;
    let mosque = setup_mosque(&db).await;

    let event_date = Utc::now()
        .with_timezone(&FixedOffset::east_opt(0).unwrap())
        + Duration::days(7);

    let create_event = CreateEvent {
        title: "Weekly Halaqah".to_string(),
        description: "A weekly gathering for Quran study and discussion.".to_string(),
        category: EventCategory::Halaqah,
        date: event_date,
        mosque: mosque.id.clone(),
        speaker: Some("Imam Ahmed".to_string()),
        recurrence_pattern: Some(EventRecurrence::Weekly),
        recurrence_duration: Some(Interval::ThreeMonths),
    };

    let response = create_event_via_api(&client, &addr, &session, auth_method, &mosque.id.to_string(), create_event).await;

    assert!(response.error.is_none(), "Unexpected error: {:?}", response.error);
    assert!(response.data.is_some());

    let events: Vec<Event> = db.query("SELECT * FROM events WHERE title = $title")
        .bind(("title", "Weekly Halaqah"))
        .await
        .expect("Failed to query events")
        .take(0)
        .expect("Take failed");

    assert_eq!(events.len(), 1);
    let event = &events[0];
    assert_eq!(event.recurrence_pattern, Some(EventRecurrence::Weekly));
    assert!(event.recurrence_end_date.is_some());
    
    let expected_end_date = event_date + Duration::days(90);
    let end_date_diff = (event.recurrence_end_date.unwrap() - expected_end_date).num_hours().abs();
    assert!(end_date_diff < 24, "End date should be approximately 90 days from start");
}

#[tokio::test]
async fn test_create_one_time_event_via_api() {
    let db = get_test_db().await;
    let addr = spawn_app(db.clone());
    let client = Client::new();

    let (_user, session) = setup_user_and_session(&db).await;
    let mosque = setup_mosque(&db).await;

    let event_date = Utc::now()
        .with_timezone(&FixedOffset::east_opt(0).unwrap())
        + Duration::days(7);

    let create_event = CreateEvent {
        title: "One-time Lecture".to_string(),
        description: "A special lecture on Islamic history.".to_string(),
        category: EventCategory::Lecture,
        date: event_date,
        mosque: mosque.id.clone(),
        speaker: Some("Scholar Yusuf".to_string()),
        recurrence_pattern: None,
        recurrence_duration: None,
    };

    let response = create_event_via_api(&client, &addr, &session, AuthMethod::Mobile, &mosque.id.to_string(), create_event).await;

    assert!(response.error.is_none(), "Unexpected error: {:?}", response.error);

    let events: Vec<Event> = db.query("SELECT * FROM events WHERE title = $title")
        .bind(("title", "One-time Lecture"))
        .await
        .expect("Failed to query events")
        .take(0)
        .expect("Take failed");

    assert_eq!(events.len(), 1);
    let event = &events[0];
    assert!(event.recurrence_pattern.is_none());
    assert!(event.recurrence_end_date.is_none());
}

#[rstest]
#[case::daily(EventRecurrence::Daily, Some(Interval::OneMonth))]
#[case::weekly(EventRecurrence::Weekly, Some(Interval::ThreeMonths))]
#[case::biweekly(EventRecurrence::Biweekly, Some(Interval::SixMonths))]
#[case::monthly(EventRecurrence::Monthly, Some(Interval::OneYear))]
#[case::indefinite(EventRecurrence::Weekly, Some(Interval::Indefinite))]
#[tokio::test]
async fn test_create_event_with_different_recurrence_patterns(
    #[case] pattern: EventRecurrence,
    #[case] duration: Option<Interval>,
) {
    let db = get_test_db().await;
    let addr = spawn_app(db.clone());
    let client = Client::new();

    let (_user, session) = setup_user_and_session(&db).await;
    let mosque = setup_mosque(&db).await;

    let event_date = Utc::now()
        .with_timezone(&FixedOffset::east_opt(0).unwrap())
        + Duration::days(7);

    let title = format!("{:?} Event", pattern);
    let create_event = CreateEvent {
        title: title.clone(),
        description: "Test event".to_string(),
        category: EventCategory::Community,
        date: event_date,
        mosque: mosque.id.clone(),
        speaker: None,
        recurrence_pattern: Some(pattern.clone()),
        recurrence_duration: duration,
    };

    let response = create_event_via_api(&client, &addr, &session, AuthMethod::Mobile, &mosque.id.to_string(), create_event).await;
    assert!(response.error.is_none(), "Unexpected error: {:?}", response.error);

    let events: Vec<Event> = db.query("SELECT * FROM events WHERE title = $title")
        .bind(("title", title))
        .await
        .expect("Failed to query events")
        .take(0)
        .expect("Take failed");

    assert_eq!(events.len(), 1);
    assert_eq!(events[0].recurrence_pattern, Some(pattern));
    assert!(events[0].recurrence_end_date.is_some());
}

#[tokio::test]
async fn test_update_event_title() {
    let db = get_test_db().await;
    let addr = spawn_app(db.clone());
    let client = Client::new();

    let (_user, session) = setup_user_and_session(&db).await;
    let mosque = setup_mosque(&db).await;

    let event_date = Utc::now()
        .with_timezone(&FixedOffset::east_opt(0).unwrap())
        + Duration::days(7);

    let create_event = CreateEvent {
        title: "Original Title".to_string(),
        description: "Original description".to_string(),
        category: EventCategory::Lecture,
        date: event_date,
        mosque: mosque.id.clone(),
        speaker: None,
        recurrence_pattern: None,
        recurrence_duration: None,
    };

    let _ = create_event_via_api(&client, &addr, &session, AuthMethod::Mobile, &mosque.id.to_string(), create_event).await;

    let events: Vec<Event> = db.query("SELECT * FROM events WHERE title = $title")
        .bind(("title", "Original Title"))
        .await
        .expect("Failed to query events")
        .take(0)
        .expect("Take failed");

    let event_id = events[0].id.clone();

    let update_url = format!("{}/mosques/events/update-event", addr);
    let update_params = UpdateEventParams {
        event_id: event_id.to_string(),
        updated_event: UpdatedEvent {
            title: Some("Updated Title".to_string()),
            description: None,
            category: None,
            date: None,
            mosque: None,
            speaker: None,
            recurrence_pattern: None,
            recurrence_end_date: None,
        },
    };

    let req = build_auth_patch(&client, &session, AuthMethod::Mobile, &update_url);
    let response = req.json(&update_params).send().await.expect("Failed to send update");

    assert!(response.status().is_success(), "Update failed: {:?}", response.text().await);

    let updated_events: Vec<Event> = db.query("SELECT * FROM $event_id")
        .bind(("event_id", event_id))
        .await
        .expect("Failed to query updated event")
        .take(0)
        .expect("Take failed");

    assert_eq!(updated_events[0].title, "Updated Title");
}

#[tokio::test]
async fn test_delete_event() {
    let db = get_test_db().await;
    let addr = spawn_app(db.clone());
    let client = Client::new();

    let (_user, session) = setup_user_and_session(&db).await;
    let mosque = setup_mosque(&db).await;

    let event_date = Utc::now()
        .with_timezone(&FixedOffset::east_opt(0).unwrap())
        + Duration::days(7);

    let create_event = CreateEvent {
        title: "Event to Delete".to_string(),
        description: "This event will be deleted".to_string(),
        category: EventCategory::Community,
        date: event_date,
        mosque: mosque.id.clone(),
        speaker: None,
        recurrence_pattern: None,
        recurrence_duration: None,
    };

    let _ = create_event_via_api(&client, &addr, &session, AuthMethod::Mobile, &mosque.id.to_string(), create_event).await;

    let events: Vec<Event> = db.query("SELECT * FROM events WHERE title = $title")
        .bind(("title", "Event to Delete"))
        .await
        .expect("Failed to query events")
        .take(0)
        .expect("Take failed");

    assert!(!events.is_empty(), "No events found with title 'Event to Delete'");
    
    let event_id = events[0].id.clone();
    let event_id_str = event_id.to_string();
    eprintln!("Event ID: {}", event_id_str);
    
    let encoded_event_id = urlencoding::encode(&event_id_str);

    let delete_url = format!("{}/mosques/events/delete/?event_id={}", addr, encoded_event_id);
    let req = build_auth_delete(&client, &session, AuthMethod::Mobile, &delete_url);
    let response = req.send().await.expect("Failed to send delete");

    if !response.status().is_success() {
        let body = response.text().await.expect("Failed to read body");
        panic!("Delete failed with status: {}", body);
    }
    assert!(response.status().is_success());

    let api_response: ApiResponse<String> = response.json().await.expect("Failed to deserialize");
    assert!(api_response.error.is_none());
    assert_eq!(api_response.data, Some("Successfully deleted the event record".to_string()));

    let deleted_events: Vec<Event> = db.query("SELECT * FROM $event_id")
        .bind(("event_id", event_id))
        .await
        .expect("Failed to query deleted event")
        .take(0)
        .expect("Take failed");

    assert!(deleted_events.is_empty(), "Event should be deleted");
}

#[tokio::test]
async fn test_manual_rotation_trigger() {
    let db = get_test_db().await;

    let mosque: MosqueRecord = db.create("mosques")
        .content(CreateMosque {
            location: Geometry::Point((0.0, 0.0).into()),
            name: "Rotation Test Mosque".to_string(),
        })
        .await
        .expect("Failed to create mosque")
        .expect("Not returned");

    let past_date = Utc::now()
        .with_timezone(&FixedOffset::east_opt(0).unwrap())
        - Duration::days(1);

    let event: Event = db.create("events")
        .content(merzah::models::events::EventRecord {
            title: "Past Weekly Event".to_string(),
            description: "This event should rotate".to_string(),
            category: EventCategory::Halaqah,
            date: past_date,
            mosque: mosque.id.clone(),
            speaker: None,
            recurrence_pattern: Some(EventRecurrence::Weekly),
            recurrence_end_date: Some(past_date + Duration::days(365)),
        })
        .await
        .expect("Failed to create event")
        .expect("Not returned");

    let original_date = event.date;
    let rotated_count = check_and_rotate_events(&db).await.expect("Failed to rotate events");

    assert_eq!(rotated_count, 1);

    let rotated_events: Vec<Event> = db.query("SELECT * FROM $event_id")
        .bind(("event_id", event.id.clone()))
        .await
        .expect("Failed to query rotated event")
        .take(0)
        .expect("Take failed");

    assert_eq!(rotated_events.len(), 1);
    let rotated_event = &rotated_events[0];
    
    let expected_next = calculate_next_date(original_date, EventRecurrence::Weekly).unwrap();
    assert_eq!(rotated_event.date, expected_next);
}

#[tokio::test]
async fn test_rsvp_persistence_across_rotation() {
    let db = get_test_db().await;

    let user_id = RecordId::from(("users", "rsvp_user"));
    let user: User = db
        .create(user_id.clone())
        .content(User {
            id: user_id.clone(),
            created_at: Datetime::default(),
            display_name: "RSVP User".to_string(),
            password_hash: "hash".to_string(),
            role: "regular".to_string(),
            updated_at: Datetime::default(),
        })
        .await
        .expect("Failed to create user")
        .expect("Not returned");

    let mosque: MosqueRecord = db.create("mosques")
        .content(CreateMosque {
            location: Geometry::Point((0.0, 0.0).into()),
            name: "RSVP Test Mosque".to_string(),
        })
        .await
        .expect("Failed to create mosque")
        .expect("Not returned");

    let past_date = Utc::now()
        .with_timezone(&FixedOffset::east_opt(0).unwrap())
        - Duration::days(1);

    let event: Event = db.create("events")
        .content(merzah::models::events::EventRecord {
            title: "RSVP Rotation Event".to_string(),
            description: "Test RSVP persistence".to_string(),
            category: EventCategory::Halaqah,
            date: past_date,
            mosque: mosque.id.clone(),
            speaker: None,
            recurrence_pattern: Some(EventRecurrence::Weekly),
            recurrence_end_date: Some(past_date + Duration::days(365)),
        })
        .await
        .expect("Failed to create event")
        .expect("Not returned");

    db.query("RELATE $user -> attending -> $event")
        .bind(("user", user.id.clone()))
        .bind(("event", event.id.clone()))
        .await
        .expect("Failed to create RSVP");

    let rsvp_before: Vec<RecordId> = db.query("SELECT in FROM attending WHERE out = $event")
        .bind(("event", event.id.clone()))
        .await
        .expect("Failed to query RSVP before rotation")
        .take(0)
        .expect("Take failed");
    assert_eq!(rsvp_before.len(), 1);

    let _ = check_and_rotate_events(&db).await.expect("Failed to rotate events");

    let rsvp_after: Vec<RecordId> = db.query("SELECT in FROM attending WHERE out = $event")
        .bind(("event", event.id.clone()))
        .await
        .expect("Failed to query RSVP after rotation")
        .take(0)
        .expect("Take failed");
    assert_eq!(rsvp_after.len(), 1, "RSVP should persist after rotation");
}

#[tokio::test]
async fn test_rotation_deletes_event_past_end_date() {
    let db = get_test_db().await;

    let mosque: MosqueRecord = db.create("mosques")
        .content(CreateMosque {
            location: Geometry::Point((0.0, 0.0).into()),
            name: "End Date Test Mosque".to_string(),
        })
        .await
        .expect("Failed to create mosque")
        .expect("Not returned");

    let past_date = Utc::now()
        .with_timezone(&FixedOffset::east_opt(0).unwrap())
        - Duration::days(1);

    let end_date = past_date + Duration::hours(12);

    let event: Event = db.create("events")
        .content(merzah::models::events::EventRecord {
            title: "Ended Recurring Event".to_string(),
            description: "This event has ended".to_string(),
            category: EventCategory::Halaqah,
            date: past_date,
            mosque: mosque.id.clone(),
            speaker: None,
            recurrence_pattern: Some(EventRecurrence::Weekly),
            recurrence_end_date: Some(end_date),
        })
        .await
        .expect("Failed to create event")
        .expect("Not returned");

    let _ = check_and_rotate_events(&db).await.expect("Failed to rotate events");

    let remaining_events: Vec<Event> = db.query("SELECT * FROM $event_id")
        .bind(("event_id", event.id.clone()))
        .await
        .expect("Failed to query event")
        .take(0)
        .expect("Take failed");

    assert!(remaining_events.is_empty(), "Event should be deleted when next date exceeds end date");
}

#[tokio::test]
async fn test_query_returns_correct_events_not_rotated_yet() {
    let db = get_test_db().await;

    let mosque: MosqueRecord = db.create("mosques")
        .content(CreateMosque {
            location: Geometry::Point((0.0, 0.0).into()),
            name: "Query Test Mosque".to_string(),
        })
        .await
        .expect("Failed to create mosque")
        .expect("Not returned");

    let future_date = Utc::now()
        .with_timezone(&FixedOffset::east_opt(0).unwrap())
        + Duration::days(7);

    let event: Event = db.create("events")
        .content(merzah::models::events::EventRecord {
            title: "Future Event".to_string(),
            description: "This event is in the future".to_string(),
            category: EventCategory::Halaqah,
            date: future_date,
            mosque: mosque.id.clone(),
            speaker: None,
            recurrence_pattern: Some(EventRecurrence::Weekly),
            recurrence_end_date: Some(future_date + Duration::days(90)),
        })
        .await
        .expect("Failed to create event")
        .expect("Not returned");

    let rotated_count = check_and_rotate_events(&db).await.expect("Failed to check rotation");
    assert_eq!(rotated_count, 0, "Future event should not be rotated");

    let events: Vec<Event> = db.query("SELECT * FROM $event_id")
        .bind(("event_id", event.id.clone()))
        .await
        .expect("Failed to query event")
        .take(0)
        .expect("Take failed");

    assert_eq!(events.len(), 1);
    assert_eq!(events[0].date, future_date);
}

#[tokio::test]
async fn test_non_recurring_event_not_rotated() {
    let db = get_test_db().await;

    let mosque: MosqueRecord = db.create("mosques")
        .content(CreateMosque {
            location: Geometry::Point((0.0, 0.0).into()),
            name: "Non-recurring Test Mosque".to_string(),
        })
        .await
        .expect("Failed to create mosque")
        .expect("Not returned");

    let past_date = Utc::now()
        .with_timezone(&FixedOffset::east_opt(0).unwrap())
        - Duration::days(1);

    let event: Event = db.create("events")
        .content(merzah::models::events::EventRecord {
            title: "Past Non-recurring Event".to_string(),
            description: "This event is not recurring".to_string(),
            category: EventCategory::Halaqah,
            date: past_date,
            mosque: mosque.id.clone(),
            speaker: None,
            recurrence_pattern: None,
            recurrence_end_date: None,
        })
        .await
        .expect("Failed to create event")
        .expect("Not returned");

    let rotated_count = check_and_rotate_events(&db).await.expect("Failed to check rotation");
    assert_eq!(rotated_count, 0, "Non-recurring event should not be rotated");

    let events: Vec<Event> = db.query("SELECT * FROM $event_id")
        .bind(("event_id", event.id.clone()))
        .await
        .expect("Failed to query event")
        .take(0)
        .expect("Take failed");

    assert_eq!(events.len(), 1);
    assert_eq!(events[0].date, past_date, "Non-recurring event date should remain unchanged");
}
