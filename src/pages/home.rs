use crate::components::sections::NearbyMosquesSection;
use crate::components::sections::RecommendedSection;
use crate::components::sections::TopTwoDailyPrayersSections;
use crate::components::sections::UpcomingEventsSection;
use leptos::IntoView;
use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="min-w-0 space-y-8 mt-4 mb-4">
            <TopTwoDailyPrayersSections />

            <section class="min-w-0 space-y-5">
                <div class="flex flex-wrap items-center justify-between gap-2">
                    <h2 class="min-w-0 text-xl font-bold text-foreground-900 sm:text-2xl">"Nearby Mosques"</h2>
                    <A
                        href="/mosques"
                        attr:class="inline-flex min-h-[44px] shrink-0 items-center whitespace-nowrap text-sm font-medium text-sky-600 dark:text-sky-400 hover:text-sky-700 dark:hover:text-sky-300 sm:text-base"
                    >
                        "View All →"
                    </A>
                </div>
                <NearbyMosquesSection />
            </section>

            <section class="min-w-0 space-y-5">
                <div class="flex flex-wrap items-center justify-between gap-2">
                    <h2 class="min-w-0 text-xl font-bold text-foreground-900 sm:text-2xl">"Upcoming Events"</h2>
                    <A
                        href="/events"
                        attr:class="inline-flex min-h-[44px] shrink-0 items-center whitespace-nowrap text-sm font-medium text-sky-600 dark:text-sky-400 hover:text-sky-700 dark:hover:text-sky-300 sm:text-base"
                    >
                        "View All →"
                    </A>
                </div>
                <UpcomingEventsSection />
            </section>

            <section class="min-w-0 space-y-5">
                <div class="flex flex-wrap items-center justify-between gap-2">
                    <h2 class="min-w-0 text-xl font-bold text-foreground-900 sm:text-2xl">"Recommended"</h2>
                    <A
                        href="/learn"
                        attr:class="inline-flex min-h-[44px] shrink-0 items-center whitespace-nowrap text-sm font-medium text-sky-600 dark:text-sky-400 hover:text-sky-700 dark:hover:text-sky-300 sm:text-base"
                    >
                        "View All →"
                    </A>
                </div>
                <RecommendedSection />
            </section>
        </div>
    }
}
