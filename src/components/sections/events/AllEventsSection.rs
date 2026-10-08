use leptos::IntoView;
use leptos::prelude::*;

use crate::components::cards::AllEventCard;
use crate::models::event_page_item::EventPageItem;

const FEATURED_COUNT: usize = 2;

#[component]
pub fn AllEventsSection(events: Memo<Vec<EventPageItem>>) -> impl IntoView {
    view! {
        <div class="grid gap-8 md:grid-cols-2 xl:grid-cols-3">
            <ForEnumerate
                each=move || {
                    events.get().into_iter().skip(FEATURED_COUNT).collect::<Vec<_>>()
                }
                key=|item| item.id.clone()
                let(idx, item)
            >
                <AllEventCard
                    index=idx.get()
                    category=item.category.clone()
                    distance="--".to_string()
                    title=item.title.clone()
                    mosque=item.mosque_name.clone()
                    date=item.date_all.clone()
                    description=item.description.clone()
                    cta_label="View Details".to_string()
                />
            </ForEnumerate>
            <Show when=move || {
                events.get().len() <= FEATURED_COUNT
            }>
                <p class="text-sm text-foreground-600">"No more events near you yet."</p>
            </Show>
        </div>
    }
}
