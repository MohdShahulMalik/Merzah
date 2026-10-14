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
        <div class="min-w-0 space-y-8 mt-4 mb-4">
            <TopTwoDailyPrayersSections />

            <section class="min-w-0 space-y-5">
                <div class="flex flex-wrap items-center justify-between gap-2">
                    <h2 class="min-w-0 text-xl font-bold text-purple-900 sm:text-2xl">"Nearby Mosques"</h2>
                    <button
                        class="min-h-[44px] shrink-0 whitespace-nowrap text-sm font-medium text-purple-600 hover:text-purple-700 sm:text-base"
                        on:click=move |_| set_show_all.update(|value| *value = !*value)
                    >
                        {move || if show_all.get() { "Show less" } else { "View All →" }}
                    </button>
                </div>
                <NearbyMosquesSection show_all=show_all />
            </section>

            <section class="min-w-0 space-y-5">
                <div class="flex flex-wrap items-center justify-between gap-2">
                    <h2 class="min-w-0 text-xl font-bold text-purple-900 sm:text-2xl">"Upcoming Events"</h2>
                    <button
                        class="min-h-[44px] shrink-0 whitespace-nowrap text-sm font-medium text-purple-600 hover:text-purple-700 sm:text-base"
                        on:click=move |_| set_show_all_events.update(|value| *value = !*value)
                    >
                        {move || if show_all_events.get() { "Show less" } else { "View All →" }}
                    </button>
                </div>
                <UpcomingEventsSection show_all=show_all_events />
            </section>

            <section class="min-w-0 space-y-5">
                <div class="flex flex-wrap items-center justify-between gap-2">
                    <h2 class="min-w-0 text-xl font-bold text-purple-900 sm:text-2xl">"Recommended"</h2>
                    <button
                        class="min-h-[44px] shrink-0 whitespace-nowrap text-sm font-medium text-purple-600 hover:text-purple-700 sm:text-base"
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
