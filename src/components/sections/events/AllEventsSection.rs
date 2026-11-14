use leptos::IntoView;
use leptos::prelude::*;

use crate::components::cards::AllEventCard;
use crate::models::event_page_item::EventPageItem;

const FEATURED_COUNT: usize = 2;
const ALL_EVENTS_CAP: usize = 6;

#[component]
pub fn AllEventsSection(events: Vec<EventPageItem>) -> impl IntoView {
    let rest = events.into_iter().skip(FEATURED_COUNT).collect::<Vec<_>>();
    let empty = rest.is_empty();
    let hidden_count = rest.len().saturating_sub(ALL_EVENTS_CAP);
    let expandable = hidden_count > 0;
    let show_all = RwSignal::new(false);

    view! {
        <div class="grid min-w-0 gap-5 sm:grid-cols-2 sm:gap-8 xl:grid-cols-3">
            <ForEnumerate
                each=move || {
                    if show_all.get() {
                        rest.clone()
                    } else {
                        rest.iter().take(ALL_EVENTS_CAP).cloned().collect::<Vec<_>>()
                    }
                }
                key=|item| item.id.clone()
                let(idx, item)
            >
                <AllEventCard
                    index=idx.get()
                    id=item.id.clone()
                    category=item.category.clone()
                    distance="--".to_string()
                    title=item.title.clone()
                    mosque=item.mosque_name.clone()
                    date=item.date_all.clone()
                    description=item.description.clone()
                    cta_label="View Details".to_string()
                />
            </ForEnumerate>
            <Show when=move || empty>
                <p class="text-sm text-foreground-600">"No more events near you yet."</p>
            </Show>
        </div>
        <Show when=move || expandable>
            <button
                type="button"
                class="inline-flex min-h-[44px] cursor-pointer items-center whitespace-nowrap font-medium text-sky-600 dark:text-sky-400 hover:text-sky-700 dark:hover:text-sky-300"
                on:click=move |_| show_all.update(|expanded| *expanded = !*expanded)
            >
                {move || if show_all.get() {
                    "Show Less".to_string()
                } else {
                    format!("View All ({hidden_count} more)")
                }}
            </button>
        </Show>
    }
}
