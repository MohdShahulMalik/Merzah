use crate::components::sections::NearbyMosquesSection;
use crate::components::sections::RecommendedSection;
use crate::components::sections::TopTwoDailyPrayersSections;
use crate::components::sections::UpcomingEventsSection;
use leptos::IntoView;
use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    let (show_all, set_show_all) = signal(false);
    let (show_all_events, set_show_all_events) = signal(false);
    let (show_all_courses, set_show_all_courses) = signal(false);

    view! {
        <div class="space-y-8 mt-4 mr-4 mb-4">
            <TopTwoDailyPrayersSections />

            <section class="space-y-5">
                <div class="flex items-center justify-between">
                    <h2 class="text-2xl font-bold text-purple-900">"Nearby Mosques"</h2>
                    <button
                        class="font-medium text-purple-600 hover:text-purple-700"
                        on:click=move |_| set_show_all.update(|value| *value = !*value)
                    >
                        {move || if show_all.get() { "Show less" } else { "View All →" }}
                    </button>
                </div>
                <NearbyMosquesSection show_all=show_all />
            </section>

            <section class="space-y-5">
                <div class="flex items-center justify-between">
                    <h2 class="text-2xl font-bold text-purple-900">"Upcoming Events"</h2>
                    <button
                        class="font-medium text-purple-600 hover:text-purple-700"
                        on:click=move |_| set_show_all_events.update(|value| *value = !*value)
                    >
                        {move || if show_all_events.get() { "Show less" } else { "View All →" }}
                    </button>
                </div>
                <UpcomingEventsSection show_all=show_all_events />
            </section>

            <section class="space-y-5">
                <div class="flex items-center justify-between">
                    <h2 class="text-2xl font-bold text-purple-900">"Recommended"</h2>
                    <button
                        class="font-medium text-purple-600 hover:text-purple-700"
                        on:click=move |_| set_show_all_courses.update(|value| *value = !*value)
                    >
                        {move || if show_all_courses.get() { "Show less" } else { "View All →" }}
                    </button>
                </div>
                <RecommendedSection show_all=show_all_courses />
            </section>
        </div>
    }
}
