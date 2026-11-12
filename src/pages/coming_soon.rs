use leptos::IntoView;
use leptos::prelude::*;
use leptos_router::components::A;

/// Generic placeholder shown for routes whose content has not been built yet.
#[component]
pub fn ComingSoon(title: &'static str, description: &'static str) -> impl IntoView {
    view! {
        <div class="flex min-w-0 flex-col items-center justify-center gap-4 py-16 text-center sm:py-24">
            <h1 class="font-serif text-3xl font-bold text-foreground-900 sm:text-4xl text-balance">
                {title}
            </h1>
            <p class="max-w-md text-base leading-7 text-foreground-700">
                {description}
            </p>
            <p class="text-sm font-semibold uppercase tracking-[0.2em] text-sky-600 dark:text-sky-400">
                "This page will come soon"
            </p>
            <A
                href="/home"
                attr:class="inline-flex min-h-[44px] items-center rounded-lg bg-indigo-500 px-5 py-2.5 text-sm font-bold text-white transition hover:bg-indigo-600"
            >
                "Back to Home"
            </A>
        </div>
    }
}

/// Destination for the "View All" link on the Featured Courses section.
#[component]
pub fn AllCourses() -> impl IntoView {
    view! {
        <ComingSoon
            title="All Courses"
            description="Browse the complete course catalog. We are putting the finishing touches on it."
        />
    }
}

/// Destination for the "View All" link on the Guided Learning Paths section.
#[component]
pub fn AllLearningPaths() -> impl IntoView {
    view! {
        <ComingSoon
            title="All Learning Paths"
            description="Browse every guided learning path. We are putting the finishing touches on it."
        />
    }
}

/// Destination for the event card call-to-action buttons.
#[component]
pub fn EventDetails() -> impl IntoView {
    view! {
        <div class="flex min-w-0 flex-col items-center justify-center gap-4 py-16 text-center sm:py-24">
            <h1 class="font-serif text-3xl font-bold text-foreground-900 sm:text-4xl text-balance">
                "Event Details"
            </h1>
            <p class="max-w-md text-base leading-7 text-foreground-700">
                "Full event details, including RSVPs, are not ready yet. We are putting the finishing touches on this page."
            </p>
            <p class="text-sm font-semibold uppercase tracking-[0.2em] text-sky-600 dark:text-sky-400">
                "This page will come soon"
            </p>
            <A
                href="/events"
                attr:class="inline-flex min-h-[44px] items-center rounded-lg bg-indigo-500 px-5 py-2.5 text-sm font-bold text-white transition hover:bg-indigo-600"
            >
                "Back to Events"
            </A>
        </div>
    }
}
