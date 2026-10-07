use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct UpcomingEventItem {
    pub id: String,
    pub event_title: String,
    pub event_type: String,
    pub event_type_class: String,
    pub mosque_name: String,
    pub event_day: String,
    pub event_time: String,
    pub event_short_description: String,
}

impl UpcomingEventItem {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: String,
        event_title: String,
        event_type: String,
        event_type_class: String,
        mosque_name: String,
        event_day: String,
        event_time: String,
        event_short_description: String,
    ) -> Self {
        Self {
            id,
            event_title,
            event_type,
            event_type_class,
            mosque_name,
            event_day,
            event_time,
            event_short_description,
        }
    }
}
