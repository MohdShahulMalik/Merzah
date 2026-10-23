use leptos::IntoView;
use leptos::prelude::*;

#[component]
pub fn FeaturedEventCard(
    index: usize,
    badge: String,
    title: String,
    date: String,
    location: String,
    description: String,
    cta_label: String,
) -> impl IntoView {
    let border_class = match index % 2 {
        0 => "absolute left-0 top-0 h-1 w-full bg-gradient-to-r from-indigo-500 to-sky-500",
        _ => "absolute left-0 top-0 h-1 w-full bg-gradient-to-r from-purple-500 to-indigo-500",
    };

    view! {
        <article class="relative min-w-0 card p-5 shadow-lg transition hover:-translate-y-1 hover:border-purple-200 hover:shadow-xl sm:p-6">
            <div class=border_class />

            <div class="min-w-0 space-y-4">
                <span class="inline-flex rounded-md bg-sky-50 dark:bg-sky-400/10 px-3 py-1.5 text-xs font-bold uppercase tracking-wide text-sky-700 dark:text-sky-400 ring-1 ring-sky-200/70 dark:ring-sky-400/30">
                    {badge}
                </span>

                <div class="min-w-0 space-y-3">
                    <h2 class="min-w-0 font-serif text-lg font-bold leading-tight text-foreground-900 sm:text-xl md:text-2xl text-balance break-words">
                        {title}
                    </h2>

                    <div class="flex min-w-0 flex-wrap items-center gap-x-6 gap-y-2 text-sm text-foreground-700">
                        <div class="flex items-center gap-2">
                            <span aria-hidden="true">"🗓️"</span>
                            <span>{date}</span>
                        </div>

                        <div class="flex items-center gap-2">
                            <span aria-hidden="true">"📍"</span>
                            <span>{location}</span>
                        </div>
                    </div>
                </div>

                <p class="min-w-0 max-w-3xl text-sm leading-6 text-foreground-700 break-words">
                    {description}
                </p>

                <a
                    href="#"
                    class="inline-flex min-h-[44px] items-center rounded-lg bg-violet-600 px-5 py-2.5 text-sm font-bold text-white transition hover:bg-violet-700"
                >
                    {cta_label}" →"
                </a>
            </div>
        </article>
    }
}

#[component]
pub fn AllEventCard(
    index: usize,
    category: String,
    distance: String,
    title: String,
    mosque: String,
    date: String,
    description: String,
    cta_label: String,
) -> impl IntoView {
    let border_class = match index % 3 {
        0 => {
            "absolute bottom-0 left-0 top-0 w-1 bg-violet-600 transition-all duration-200 group-hover:w-1.5"
        }
        1 => {
            "absolute bottom-0 left-0 top-0 w-1 bg-fuchsia-500 transition-all duration-200 group-hover:w-1.5"
        }
        _ => {
            "absolute bottom-0 left-0 top-0 w-1 bg-sky-500 transition-all duration-200 group-hover:w-1.5"
        }
    };

    let category_pill_class = match index % 3 {
        0 => {
            "inline-flex rounded-md bg-sky-50 dark:bg-sky-400/10 px-2 py-0.5 text-[0.65rem] font-semibold uppercase tracking-[0.14em] text-sky-700 dark:text-sky-400 ring-1 ring-sky-200/70 dark:ring-sky-400/30"
        }
        1 => {
            "inline-flex rounded-md bg-fuchsia-50 px-2 py-0.5 text-[0.65rem] font-semibold uppercase tracking-[0.14em] text-fuchsia-700 ring-1 ring-fuchsia-200/70"
        }
        _ => {
            "inline-flex rounded-md bg-sky-50 px-2 py-0.5 text-[0.65rem] font-semibold uppercase tracking-[0.14em] text-sky-700 ring-1 ring-sky-200/70"
        }
    };

    view! {
        <article class="group relative min-w-0 card p-5 shadow-lg transition duration-200 ease-out hover:-translate-y-1 hover:border-purple-200 hover:shadow-xl sm:p-6">
            <div class=border_class />

            <div class="min-w-0 space-y-5 pl-2">
                <div class="flex min-w-0 flex-wrap items-center justify-between gap-2">
                    <span class=category_pill_class>
                        {category}
                    </span>

                    <div class="flex shrink-0 items-center gap-2 text-sm text-foreground-600">
                        <span aria-hidden="true">"📍"</span>
                        <span>{distance}</span>
                    </div>
                </div>

                <div class="min-w-0 space-y-3">
                    <h2 class="min-w-0 font-serif text-lg font-bold leading-tight text-foreground-900 sm:text-xl text-balance break-words">
                        {title}
                    </h2>

                    <p class="text-sm font-semibold text-foreground-700">
                        {mosque}
                    </p>

                    <div class="flex items-center gap-2 text-sm font-bold text-foreground-900">
                        <span aria-hidden="true">"⏱️"</span>
                        <span>{date}</span>
                    </div>
                </div>

                <p class="line-clamp-2 min-w-0 text-sm leading-6 text-foreground-700 break-words">
                    {description}
                </p>

                <a
                    href="#"
                    class="inline-flex min-h-[44px] items-center rounded-lg border-2 border-sky-300/80 bg-sky-50 dark:bg-sky-400/10 px-5 py-2.5 text-sm font-bold text-sky-700 dark:text-sky-400 transition hover:bg-sky-100 dark:hover:bg-sky-400/15"
                >
                    {cta_label}" →"
                </a>
            </div>
        </article>
    }
}
