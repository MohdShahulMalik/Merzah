use chrono::{DateTime, FixedOffset};
use garde::Validate;
use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use surrealdb::RecordId;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum EventCategory {
    Halaqah,
    Fundraiser,
    Youth,
    Lecture,
    Community,
    Workshop,
    Seminar,
    Conference,
    Sports,
    Social,
    Volunteer,
    Iftar,
    Taraweeh,
    Eid,
}

#[cfg(feature = "ssr")]
#[derive(Debug, Deserialize, Serialize)]
pub struct Event {
    pub id: RecordId,
    pub title: String,
    pub description: String,
    pub category: EventCategory,
    pub date: DateTime<FixedOffset>,
    pub speaker: Option<String>,
    pub recurrence_pattern: Option<EventRecurrence>,
    pub recurrence_end_date: Option<DateTime<FixedOffset>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum EventRecurrence {
    Daily,
    Weekly,
    Biweekly,
    Weekdays,
    Weekends,
    Monthly,
    Quaterly,
    Yearly,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Interval {
    OneMonth,
    ThreeMonths,
    SixMonths,
    OneYear,
    Indefinite,
}

#[derive(Debug, Validate, Deserialize, Serialize, Clone)]
pub struct CreateEvent {
    #[garde(length(min = 2, max = 100))]
    pub title: String,
    #[garde(length(min = 10, max = 1000))]
    pub description: String,
    #[garde(skip)]
    pub category: EventCategory,
    #[garde(skip)]
    pub date: DateTime<FixedOffset>,
    #[garde(skip)]
    pub mosque: RecordId,
    #[garde(length(min = 2, max = 100))]
    pub speaker: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[garde(skip)]
    pub recurrence_pattern: Option<EventRecurrence>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[garde(skip)]
    pub recurrence_duration: Option<Interval>,
}

impl From<CreateEvent> for EventRecord {
    fn from(create: CreateEvent) -> Self {
        let recurrence_end_date = match create.recurrence_duration {
            Some(Interval::OneMonth) => Some(create.date + chrono::Duration::days(30)),
            Some(Interval::ThreeMonths) => Some(create.date + chrono::Duration::days(90)),
            Some(Interval::SixMonths) => Some(create.date + chrono::Duration::days(180)),
            Some(Interval::OneYear) => Some(create.date + chrono::Duration::days(365)),
            Some(Interval::Indefinite) => Some(create.date + chrono::Duration::days(365 * 100)),
            None => None,
        };

        Self {
            title: create.title,
            description: create.description,
            category: create.category,
            date: create.date,
            mosque: create.mosque,
            speaker: create.speaker,
            recurrence_pattern: create.recurrence_pattern,
            recurrence_end_date,
        }
    }
}

#[cfg(feature = "ssr")]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EventRecord {
    pub title: String,
    pub description: String,
    pub category: EventCategory,
    pub date: DateTime<FixedOffset>,
    pub mosque: RecordId,
    pub speaker: Option<String>,
    pub recurrence_pattern: Option<EventRecurrence>,
    pub recurrence_end_date: Option<DateTime<FixedOffset>>,
}

#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct UpdatedEvent {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[garde(inner(length(min = 2, max = 100)))]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[garde(inner(length(min = 10, max = 1000)))]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[garde(skip)]
    pub category: Option<EventCategory>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[garde(skip)]
    pub date: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[garde(skip)]
    pub mosque: Option<RecordId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[garde(inner(length(min = 2, max = 100)))]
    pub speaker: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[garde(skip)]
    pub recurrence_pattern: Option<EventRecurrence>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[garde(skip)]
    pub recurrence_end_date: Option<DateTime<FixedOffset>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PersonalEvent {
    pub event: Event,
    pub rsvp: bool,
}

impl PersonalEvent {
    pub fn new(event: Event, rsvp: bool) -> Self {
        Self { event, rsvp }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EventSummary {
    pub event: Event,
    pub rsvp_count: usize,
}

impl EventSummary {
    pub fn new(event: Event, rsvp_count: usize) -> Self {
        Self { event, rsvp_count }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FetchedEvents {
    Summary(Vec<EventSummary>),
    Personal(Vec<PersonalEvent>),
}
