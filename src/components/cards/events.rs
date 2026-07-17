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
    let border_colors = ["primary", "secondary"];
    let border_color = border_colors[index % border_colors.len()];

    view! {
        <article class="relative overflow-hidden rounded-xl bg-white p-6 shadow-lg hover:-translate-y-1 hover:scale-[1.02] hover:shadow-xl transition">
            <div class=format!("absolute left-0 top-0 h-1 w-full bg-{}", border_color) />

            <div class="space-y-4">
                <span class="inline-flex rounded-md bg-secondary/10 px-3 py-1.5 text-xs font-bold uppercase tracking-wide text-secondary">
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
                    class="inline-flex rounded-lg bg-primary px-5 py-2.5 text-sm font-bold text-white transition hover:bg-primary-dark"
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
        0 => "absolute bottom-0 left-0 top-0 w-1 bg-secondary transition-all duration-200 group-hover:w-1.5",
        1 => "absolute bottom-0 left-0 top-0 w-1 bg-primary transition-all duration-200 group-hover:w-1.5",
        _ => "absolute bottom-0 left-0 top-0 w-1 bg-info transition-all duration-200 group-hover:w-1.5",
    };

    view! {
        // NOTE: `group` is not a tailwind class and it's just used to trigger resize of the left border when hovering the card
        <article class="group relative overflow-hidden rounded-xl bg-white p-6 shadow-lg transition duration-200 ease-out hover:-translate-y-1 hover:shadow-xl">
            <div class=border_class />

            <div class="space-y-5 pl-2">
                <div class="flex items-center justify-between gap-4">
                    <span class="inline-flex rounded-md bg-gray-200/40 px-3 py-1.5 text-xs font-bold uppercase tracking-wide text-primary-dark">
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
                    class="inline-flex rounded-lg border-2 border-gray-200/40 bg-gray-200/40 px-5 py-2.5 text-sm text-foreground-900 font-bold text-primary-dark transition"
                >
                    {cta_label}" →"
                </a>
            </div>
        </article>
    }
}
