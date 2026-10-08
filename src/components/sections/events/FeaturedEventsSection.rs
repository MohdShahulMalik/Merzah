use leptos::IntoView;
use leptos::prelude::*;

use crate::components::cards::FeaturedEventCard;
use crate::models::event_page_item::EventPageItem;

const FEATURED_COUNT: usize = 2;

#[component]
pub fn FeaturedEventsSection(events: Memo<Vec<EventPageItem>>) -> impl IntoView {
    view! {
        <div class="grid gap-8 lg:grid-cols-2">
            <ForEnumerate
                each=move || {
                    events.get().into_iter().take(FEATURED_COUNT).collect::<Vec<_>>()
                }
                key=|item| item.id.clone()
                let(idx, item)
            >
                <FeaturedEventCard
                    index=idx.get()
                    badge=item.category.clone()
                    title=item.title.clone()
                    date=item.date_featured.clone()
                    location=item.mosque_name.clone()
                    description=item.description.clone()
                    cta_label=if item.rsvp {
                        "View Details".to_string()
                    } else {
                        "RSVP Now".to_string()
                    }
                />
            </ForEnumerate>
            <Show when=move || events.get().is_empty()>
                <p class="text-sm text-foreground-600">"No featured events near you yet."</p>
            </Show>
        </div>
    }
}
