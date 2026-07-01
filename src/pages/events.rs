use crate::components::filters::{FilterGroup, FilterOption, Filters};
use leptos::IntoView;
use leptos::prelude::*;

#[component]
pub fn Events() -> impl IntoView {
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
                    <h1 class="text-3xl font-bold text-foreground-900">"Featured Events"</h1>
                    <a href="#" class="font-semibold text-primary hover:underline">"View all →"</a>
                </div>
            </section>
        </div>
    }
}
