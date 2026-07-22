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
        <article class="relative overflow-hidden rounded-xl bg-white p-6 shadow-lg transition hover:-translate-y-1 hover:scale-[1.02] hover:shadow-xl">
            <div class=border_class />

            <div class="space-y-4">
                <span class="inline-flex rounded-md bg-violet-50 px-3 py-1.5 text-xs font-bold uppercase tracking-wide text-violet-700 ring-1 ring-violet-200/70">
                    {badge}
                </span>

                <div class="space-y-3">
                    <h2 class="font-serif text-xl font-bold leading-tight text-foreground-900 md:text-2xl">
                        {title}
                    </h2>

                    <div class="flex flex-wrap items-center gap-x-6 gap-y-2 text-sm text-foreground-700">
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

                <p class="max-w-3xl text-sm leading-6 text-foreground-700">
                    {description}
                </p>

                <a
                    href="#"
                    class="inline-flex rounded-lg bg-violet-600 px-5 py-2.5 text-sm font-bold text-white transition hover:bg-violet-700"
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
        0 => "absolute bottom-0 left-0 top-0 w-1 bg-violet-600 transition-all duration-200 group-hover:w-1.5",
        1 => "absolute bottom-0 left-0 top-0 w-1 bg-fuchsia-500 transition-all duration-200 group-hover:w-1.5",
        _ => "absolute bottom-0 left-0 top-0 w-1 bg-sky-500 transition-all duration-200 group-hover:w-1.5",
    };

    let category_pill_class = match index % 3 {
        0 => "inline-flex rounded-md bg-violet-50 px-2 py-0.5 text-[0.65rem] font-semibold uppercase tracking-[0.14em] text-violet-700 ring-1 ring-violet-200/70",
        1 => "inline-flex rounded-md bg-fuchsia-50 px-2 py-0.5 text-[0.65rem] font-semibold uppercase tracking-[0.14em] text-fuchsia-700 ring-1 ring-fuchsia-200/70",
        _ => "inline-flex rounded-md bg-sky-50 px-2 py-0.5 text-[0.65rem] font-semibold uppercase tracking-[0.14em] text-sky-700 ring-1 ring-sky-200/70",
    };

    view! {
        <article class="group relative overflow-hidden rounded-xl bg-white p-6 shadow-lg transition duration-200 ease-out hover:-translate-y-1 hover:shadow-xl">
            <div class=border_class />

            <div class="space-y-5 pl-2">
                <div class="flex items-center justify-between gap-4">
                    <span class=category_pill_class>
                        {category}
                    </span>

                    <div class="flex items-center gap-2 text-sm text-foreground-600">
                        <span aria-hidden="true">"📍"</span>
                        <span>{distance}</span>
                    </div>
                </div>

                <div class="space-y-3">
                    <h2 class="font-serif text-xl font-bold leading-tight text-foreground-900">
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

                <p class="line-clamp-2 text-sm leading-6 text-foreground-700">
                    {description}
                </p>

                <a
                    href="#"
                    class="inline-flex rounded-lg border-2 border-violet-200/80 bg-violet-50 px-5 py-2.5 text-sm font-bold text-violet-700 transition hover:bg-violet-100"
                >
                    {cta_label}" →"
                </a>
            </div>
        </article>
    }
}
