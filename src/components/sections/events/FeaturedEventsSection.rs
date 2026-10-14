use leptos::IntoView;
use leptos::prelude::*;

use crate::components::cards::FeaturedEventCard;
use crate::models::event_page_item::EventPageItem;

const FEATURED_COUNT: usize = 2;

#[component]
pub fn FeaturedEventsSection(events: Vec<EventPageItem>) -> impl IntoView {
    let featured = events.into_iter().take(FEATURED_COUNT).collect::<Vec<_>>();
    let empty = featured.is_empty();

    view! {
        <div class="grid min-w-0 gap-5 sm:gap-8 lg:grid-cols-2">
            <ForEnumerate
                each=move || featured.clone()
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
            <Show when=move || empty>
                <p class="text-sm text-foreground-600">"No featured events near you yet."</p>
            </Show>
        </div>
    }
}
