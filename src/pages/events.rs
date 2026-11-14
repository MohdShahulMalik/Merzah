use chrono::Datelike;
use leptos::IntoView;
use leptos::prelude::*;
use leptos::reactive::spawn_local;
use reactive_stores::OptionStoreExt;
use reactive_stores::Store;

use crate::app::AppState;
use crate::app::AppStateStoreFields;
use crate::components::cards::AllEventCard;
use crate::components::filters::{FilterGroup, FilterOption, Filters, default_selection};
use crate::components::sections::AllEventsSection;
use crate::components::sections::FeaturedEventsSection;
use crate::models::event_page_item::EventPageItem;
use crate::server_functions::events::fetch_users_favorite_mosques_events;
use crate::utils::geocode::reverse_geocode;

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
            let (lat, lon) = match pos {
                Some((lat, lon)) => (Some(lat), Some(lon)),
                None => (None, None),
            };

            fetch_users_favorite_mosques_events(lat, lon)
                .await
                .ok()
                .and_then(|res| res.data)
                .unwrap_or_default()
        },
    );

    // Reverse geocodes the user's coordinates into a short
    // "City, ST" label. Falls back to "--, --" until the browser shares
    // (or refuses) the location. Done via spawn_local because reqwest's
    // wasm backend is not `Send`.
    let location_label = RwSignal::new("--, --".to_string());

    Effect::new(move |_| {
        if let Some(position) = app_state.coords().get() {
            spawn_local(async move {
                let label = reverse_geocode(position.latitude, position.longitude)
                    .await
                    .unwrap_or_else(|| "--, --".to_string());
                location_label.set(label);
            });
        }
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

    let defaults = default_selection(&filter_groups);
    let default_category = defaults.first().cloned().unwrap_or_else(|| "All".to_string());
    let default_time = defaults.get(1).cloned().unwrap_or_else(|| "This Week".to_string());
    let reset_category = default_category.clone();
    let reset_time = default_time.clone();
    let search = RwSignal::new(String::new());
    let active = RwSignal::new(defaults.clone());

    let searching = {
        let defaults = defaults.clone();
        move || {
            !search.get().trim().is_empty()
                || active.with(|selection| {
                    selection
                        .iter()
                        .zip(defaults.iter())
                        .any(|(current, default)| current != default)
                })
        }
    };

    let filtered_events = move || -> Vec<EventPageItem> {
        let query = search.get().trim().to_lowercase();
        let selection = active.get();
        let category = selection.first().cloned().unwrap_or_else(|| "All".to_string());
        let time = selection.get(1).cloned().unwrap_or_else(|| "This Week".to_string());
        let time_constrained = time != default_time;

        let now = chrono::Local::now();
        let today = now.date_naive();

        events
            .get()
            .unwrap_or_default()
            .iter()
            .filter(|personal| {
                let event = &personal.event;
                let mosque_name = event.mosque_name.clone().unwrap_or_default();
                let category_label = format!("{:?}", event.category);
                let local_date = event.date.with_timezone(&chrono::Local).date_naive();
                let local_now = event.date.with_timezone(&chrono::Local);

                let matches_query = query.is_empty()
                    || event.title.to_lowercase().contains(&query)
                    || event.description.to_lowercase().contains(&query)
                    || mosque_name.to_lowercase().contains(&query)
                    || category_label.to_lowercase().contains(&query);
                let matches_category = category == default_category
                    || category_label.eq_ignore_ascii_case(&category)
                    || format!("{} {}", event.title, event.description)
                        .to_lowercase()
                        .contains(&category.to_lowercase());
                let matches_time = !time_constrained
                    || match time.as_str() {
                        "Today" => local_date == today,
                        "This Week" => {
                            local_date >= today && local_date <= today + chrono::Duration::days(6)
                        }
                        "This Month" => {
                            local_date.year() == today.year() && local_date.month() == today.month()
                        }
                        "Upcoming" => local_now >= now,
                        _ => true,
                    };

                matches_query && matches_category && matches_time
            })
            .map(EventPageItem::from_personal)
            .collect()
    };

    view! {
        <div class="min-w-0 space-y-8 py-6 sm:space-y-12 sm:py-8">
            <Filters
                search_placeholder="Search events, topics, or mosques".to_string()
                location_label=location_label.into()
                filter_groups=filter_groups
                search=search
                active=active
            />

            {move || {
                if searching() {
                    let matches = filtered_events();
                    let count = matches.len();
                    let count_label = if count == 1 { "event" } else { "events" };
                    let reset_selection = vec![reset_category.clone(), reset_time.clone()];

                    view! {
                        <section class="min-w-0 space-y-6">
                            <div class="flex min-w-0 flex-wrap items-center justify-between gap-2">
                                <h1 class="min-w-0 text-2xl font-bold text-foreground-900 sm:text-3xl text-balance">"Search Results"</h1>
                                <span class="text-sm font-medium text-foreground-600">
                                    {format!("{count} {count_label} found")}
                                </span>
                            </div>

                            {if matches.is_empty() {
                                view! {
                                    <div class="min-w-0 rounded-2xl border border-dashed border-stroke-muted bg-surface-700 p-8 text-center shadow-sm">
                                        <p class="text-3xl" aria-hidden="true">"🔍"</p>
                                        <p class="mt-2 text-sm font-semibold text-foreground-900">
                                            "No events match your search."
                                        </p>
                                        <p class="mt-1 text-sm text-foreground-600">
                                            "Try a different search term or clear the filters."
                                        </p>
                                        <button
                                            type="button"
                                            class="mt-4 inline-flex min-h-[44px] cursor-pointer items-center font-medium text-sky-600 dark:text-sky-400 hover:text-sky-700 dark:hover:text-sky-300"
                                            on:click=move |_| {
                                                search.set(String::new());
                                                active.set(reset_selection.clone());
                                            }
                                        >
                                            "Clear filters"
                                        </button>
                                    </div>
                                }
                                    .into_any()
                            } else {
                                view! {
                                    <div class="grid min-w-0 gap-5 sm:grid-cols-2 sm:gap-8 xl:grid-cols-3">
                                        {matches
                                            .into_iter()
                                            .enumerate()
                                            .map(|(index, item)| {
                                                view! {
                                                    <AllEventCard
                                                        index=index
                                                        id=item.id.clone()
                                                        category=item.category.clone()
                                                        distance="--".to_string()
                                                        title=item.title.clone()
                                                        mosque=item.mosque_name.clone()
                                                        date=item.date_all.clone()
                                                        description=item.description.clone()
                                                        cta_label="View Details".to_string()
                                                    />
                                                }
                                            })
                                            .collect_view()}
                                    </div>
                                }
                                    .into_any()
                            }}
                        </section>
                    }
                        .into_any()
                } else {
                    view! {
                        <>
                            <section class="min-w-0 space-y-6">
                                <div class="flex min-w-0 flex-wrap items-center justify-between gap-2">
                                    <h1 class="min-w-0 text-2xl font-bold text-foreground-900 sm:text-3xl text-balance">"Featured Events"</h1>
                                </div>

                                <Suspense fallback=move || view! {
                                    <p class="text-sm text-foreground-600">"Loading featured events…"</p>
                                }>
                                    {move || {
                                        let items = events
                                            .get()
                                            .unwrap_or_default()
                                            .iter()
                                            .map(EventPageItem::from_personal)
                                            .collect::<Vec<_>>();
                                        view! { <FeaturedEventsSection events=items /> }
                                    }}
                                </Suspense>
                            </section>

                            <section class="min-w-0 space-y-6">
                                <h1 class="min-w-0 text-2xl font-bold text-foreground-900 sm:text-3xl text-balance">"All Events"</h1>

                                <Suspense fallback=move || view! {
                                    <p class="text-sm text-foreground-600">"Loading events…"</p>
                                }>
                                    {move || {
                                        let items = events
                                            .get()
                                            .unwrap_or_default()
                                            .iter()
                                            .map(EventPageItem::from_personal)
                                            .collect::<Vec<_>>();
                                        view! { <AllEventsSection events=items /> }
                                    }}
                                </Suspense>
                            </section>
                        </>
                    }
                        .into_any()
                }
            }}
        </div>
    }
}
