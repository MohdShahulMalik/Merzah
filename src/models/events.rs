use serde::{Deserialize, Serialize};
use chrono::{DateTime, FixedOffset};
#[cfg(feature = "ssr")]
use surrealdb::RecordId;

use crate::models::mosque::MosqueData;

// TODO: Add relevant only categories please
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum EventCategory {
    Prayer,
    Education,
    Social,
    Professional,
    Fundraiser
}

#[derive(Debug, Deserialize)]
pub struct Event {
    pub title: String,
    pub description: String,
    pub category: EventCategory,
    pub date: DateTime<FixedOffset>,
    pub mosque: MosqueData,
    pub speaker: Option<String>,
}

// TODO: Add data validation please
#[cfg(feature = "ssr")]
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateEvent {
    pub title: String,
    pub description: String,
    pub category: EventCategory,
    pub date: DateTime<FixedOffset>,
    pub mosque: RecordId,
    pub speaker: Option<String>,
}

// TODO: Add data validation please
#[cfg(feature = "ssr")]
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateEvent {
    pub title: Option<String>,
    pub description: Option<String>,
    pub category: Option<EventCategory>,
    pub date: Option<DateTime<FixedOffset>>,
    pub mosque: Option<RecordId>,
    pub speaker: Option<String>,
}
