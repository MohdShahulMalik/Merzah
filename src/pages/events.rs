use leptos::IntoView;
use leptos::prelude::*;
use leptos_router::components::A;
use reactive_stores::OptionStoreExt;
use reactive_stores::Store;

use crate::app::AppState;
use crate::app::AppStateStoreFields;
use crate::components::filters::{FilterGroup, FilterOption, Filters};
use crate::components::sections::AllEventsSection;
use crate::components::sections::FeaturedEventsSection;
use crate::models::event_page_item::EventPageItem;
use crate::server_functions::events::fetch_users_favorite_mosques_events;

#[component]
pub fn Events() -> impl IntoView {
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

    let all = Memo::new(move |_| {
        events
            .get()
            .unwrap_or_default()
            .iter()
            .map(EventPageItem::from_personal)
            .collect::<Vec<_>>()
    });

    let filter_groups = vec![
        FilterGroup::new(
            "Category:",
            vec![
                FilterOption::new("All", true),
                FilterOption::new("Khutbah", false),
                FilterOption::new("Lecture", false),
                FilterOption::new("Halaqah", false),
                FilterOption::new("Youth", false),
                FilterOption::new("Sisters", false),
                FilterOption::new("Workshop", false),
                FilterOption::new("Career", false),
                FilterOption::new("Charity", false),
            ],
        ),
        FilterGroup::new(
            "Time:",
            vec![
                FilterOption::new("Today", false),
                FilterOption::new("This Week", true),
                FilterOption::new("This Month", false),
                FilterOption::new("Upcoming", false),
            ],
        ),
    ];

    view! {
        <div class="mr-4 space-y-12 py-8">
            <Filters
                search_placeholder="Search events, topics, or mosques".to_string()
                location_label="Detroit, MI".to_string()
                filter_groups=filter_groups
            />

            <section class="space-y-6">
                <div class="flex items-center justify-between">
                    <h1 class="text-3xl font-bold text-purple-900">"Featured Events"</h1>
                    <A href="#" attr:class="font-medium text-purple-600 hover:text-purple-700">"View All →"</A>
                </div>

                <FeaturedEventsSection events=all />
            </section>

            <section class="space-y-6">
                <h1 class="text-3xl font-bold text-purple-900">"All Events"</h1>

                <AllEventsSection events=all />
            </section>
        </div>
    }
}
