#[cfg(feature = "ssr")]
use garde::Validate;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, FixedOffset};
#[cfg(feature = "ssr")]
use surrealdb::RecordId;

use crate::models::mosque::MosqueData;

// TODO: Add relevant only categories please
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum EventCategory {
    Deen,
    Dunya,
    Fundraiser
}

#[derive(Debug, Deserialize)]
pub struct Event {
    pub id: String,
    pub title: String,
    pub description: String,
    pub category: EventCategory,
    pub date: DateTime<FixedOffset>,
    // This field is for using the FETCH clause
    pub mosque: MosqueData,
    pub speaker: Option<String>,
}

#[cfg(feature = "ssr")]
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
}

#[cfg(feature = "ssr")]
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
}
