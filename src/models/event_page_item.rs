use crate::models::events::PersonalEvent;

#[derive(Debug, Clone, PartialEq)]
pub struct EventPageItem {
    pub id: String,
    pub title: String,
    pub category: String,
    pub mosque_name: String,
    pub date_featured: String,
    pub date_all: String,
    pub description: String,
    pub rsvp: bool,
}

impl EventPageItem {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: String,
        title: String,
        category: String,
        mosque_name: String,
        date_featured: String,
        date_all: String,
        description: String,
        rsvp: bool,
    ) -> Self {
        Self {
            id,
            title,
            category,
            mosque_name,
            date_featured,
            date_all,
            description,
            rsvp,
        }
    }

    pub fn from_personal(personal: &PersonalEvent) -> Self {
        let event = &personal.event;

        Self::new(
            event.id.clone(),
            event.title.clone(),
            format!("{:?}", event.category),
            event
                .mosque_name
                .clone()
                .unwrap_or_else(|| "Nearby mosque".to_string()),
            event.date.format("%A, %b %d · %-I:%M %p").to_string(),
            event.date.format("%a, %b %d · %-I:%M %p").to_string(),
            event.description.clone(),
            personal.rsvp,
        )
    }
}
