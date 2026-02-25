#[cfg(feature = "ssr")]
use crate::models::api_responses::ApiResponse;
use chrono::{DateTime, FixedOffset};
use garde::Validate;
use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use surrealdb::RecordId;

#[cfg(feature = "ssr")]
use crate::utils::parsing::parse_record_id;

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
    pub mosque: RecordId,
    pub speaker: Option<String>,
    pub recurrence_pattern: Option<EventRecurrence>,
    pub recurrence_end_date: Option<DateTime<FixedOffset>>,
}

// To be used on client side, where we don't have access to RecordId
#[derive(Debug, Deserialize, Serialize)]
pub struct EventDetails {
    pub id: String,
    pub title: String,
    pub description: String,
    pub category: EventCategory,
    pub date: DateTime<FixedOffset>,
    pub speaker: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
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
    pub mosque: String,
    #[garde(length(min = 2, max = 100))]
    pub speaker: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[garde(skip)]
    pub recurrence_pattern: Option<EventRecurrence>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[garde(skip)]
    pub recurrence_duration: Option<Interval>,
}

#[cfg(feature = "ssr")]
impl TryFrom<CreateEvent> for EventRecord {
    type Error = ApiResponse<String>;

    fn try_from(create: CreateEvent) -> Result<Self, Self::Error> {
        let recurrence_end_date = match create.recurrence_duration {
            Some(Interval::OneMonth) => Some(create.date + chrono::Duration::days(30)),
            Some(Interval::ThreeMonths) => Some(create.date + chrono::Duration::days(90)),
            Some(Interval::SixMonths) => Some(create.date + chrono::Duration::days(180)),
            Some(Interval::OneYear) => Some(create.date + chrono::Duration::days(365)),
            Some(Interval::Indefinite) => Some(create.date + chrono::Duration::days(365 * 100)),
            None => None,
        };

        let mosque = parse_record_id::<String>(&create.mosque, "mosque")?;

        Ok(Self {
            title: create.title,
            description: create.description,
            category: create.category,
            date: create.date,
            mosque,
            speaker: create.speaker,
            recurrence_pattern: create.recurrence_pattern,
            recurrence_end_date,
        })
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
    pub mosque: Option<String>,
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

#[cfg(feature = "ssr")]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdatedEventRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<EventCategory>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mosque: Option<RecordId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speaker: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrence_pattern: Option<EventRecurrence>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrence_end_date: Option<DateTime<FixedOffset>>,
}

#[cfg(feature = "ssr")]
impl TryFrom<UpdatedEvent> for UpdatedEventRecord {
    type Error = ApiResponse<String>;

    fn try_from(update: UpdatedEvent) -> Result<Self, Self::Error> {
        let mosque = update
            .mosque
            .map(|m| parse_record_id::<String>(&m, "mosque"))
            .transpose()?;

        Ok(Self {
            title: update.title,
            description: update.description,
            category: update.category,
            date: update.date,
            mosque,
            speaker: update.speaker,
            recurrence_pattern: update.recurrence_pattern,
            recurrence_end_date: update.recurrence_end_date,
        })
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PersonalEvent {
    pub event: EventDetails,
    pub rsvp: bool,
}

impl PersonalEvent {
    pub fn new(event: EventDetails, rsvp: bool) -> Self {
        Self { event, rsvp }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EventSummary {
    pub event: EventDetails,
    pub rsvp_count: usize,
}

impl EventSummary {
    pub fn new(event: EventDetails, rsvp_count: usize) -> Self {
        Self { event, rsvp_count }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FetchedEvents {
    Summary(Vec<EventSummary>),
    Personal(Vec<PersonalEvent>),
}
