use leptos::IntoView;
use leptos::prelude::*;
use reactive_stores::OptionStoreExt;
use reactive_stores::Store;

use crate::app::AppState;
use crate::app::AppStateStoreFields;
use crate::components::cards::MosqueEventCard;
use crate::models::events::EventCategory;
use crate::models::events::PersonalEvent;
use crate::models::upcoming_event_item::UpcomingEventItem;
use crate::server_functions::events::fetch_users_favorite_mosques_events;

const PREVIEW_SIZE: usize = 3;
const DESCRIPTION_PREVIEW_CHARS: usize = 120;

fn category_label_and_class(category: &EventCategory) -> (String, &'static str) {
    let class = match category {
        EventCategory::Halaqah => "bg-purple-100 text-purple-800",
        EventCategory::Fundraiser => "bg-amber-100 text-amber-800",
        EventCategory::Youth => "bg-lime-100 text-lime-800",
        EventCategory::Lecture => "bg-[#7debf0] text-[#064f68]",
        EventCategory::Community => "bg-[#b9f4bd] text-[#11631c]",
        EventCategory::Workshop => "bg-orange-100 text-orange-800",
        EventCategory::Seminar => "bg-sky-100 text-sky-800",
        EventCategory::Conference => "bg-indigo-100 text-indigo-800",
        EventCategory::Sports => "bg-emerald-100 text-emerald-800",
        EventCategory::Social => "bg-pink-100 text-pink-800",
        EventCategory::Volunteer => "bg-teal-100 text-teal-800",
        EventCategory::Iftar => "bg-[#d9e0ff] text-[#18206d]",
        EventCategory::Taraweeh => "bg-violet-100 text-violet-800",
        EventCategory::Eid => "bg-yellow-100 text-yellow-800",
    };

    (format!("{category:?}"), class)
}

fn short_description(description: &str) -> String {
    if description.chars().count() <= DESCRIPTION_PREVIEW_CHARS {
        description.to_string()
    } else {
        let truncated: String = description
            .chars()
            .take(DESCRIPTION_PREVIEW_CHARS)
            .collect();
        format!("{truncated}…")
    }
}

fn to_item(personal: &PersonalEvent) -> UpcomingEventItem {
    let event = &personal.event;
    let (event_type, event_type_class) = category_label_and_class(&event.category);

    UpcomingEventItem::new(
        event.id.clone(),
        event.title.clone(),
        event_type,
        event_type_class.to_string(),
        event
            .mosque_name
            .clone()
            .unwrap_or_else(|| "Nearby mosque".to_string()),
        event.date.format("%A").to_string(),
        event.date.format("%-I:%M %p").to_string(),
        short_description(&event.description),
    )
}

#[component]
pub fn UpcomingEventsSection(show_all: ReadSignal<bool>) -> impl IntoView {
    let app_state = expect_context::<Store<AppState>>();
    let user_pos = move || {
        app_state
            .coords()
            .map(|coords| coords.read().as_lat_lon())
    };

    let events = Resource::new(
        user_pos,
        move |pos: Option<(f64, f64)>| async move {
            match pos {
                Some((lat, lon)) => fetch_users_favorite_mosques_events(lat, lon)
                    .await
                    .ok()
                    .and_then(|res| res.data)
                    .unwrap_or_default(),
                None => Vec::new(),
            }
        },
    );

    let visible = Memo::new(move |_| {
        let all: Vec<UpcomingEventItem> = events
            .get()
            .unwrap_or_default()
            .iter()
            .map(to_item)
            .collect();

        if show_all.get() {
            all
        } else {
            all.into_iter().take(PREVIEW_SIZE).collect()
        }
    });

    view! {
        <div class="flex gap-5 overflow-x-scroll pb-4">
            <For
                each=move || visible.get()
                key=|item| item.id.clone()
                let(item)
            >
                <MosqueEventCard
                    event_title=item.event_title.clone()
                    event_type=item.event_type.clone()
                    event_type_class=item.event_type_class.clone()
                    mosque_name=item.mosque_name.clone()
                    event_day=item.event_day.clone()
                    event_time=item.event_time.clone()
                    event_short_description=item.event_short_description.clone()
                />
            </For>
            <Show when=move || visible.get().is_empty()>
                <p class="text-sm text-foreground-600">"No upcoming events near you yet."</p>
            </Show>
        </div>
    }
}
