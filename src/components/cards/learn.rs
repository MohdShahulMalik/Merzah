use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn ContinueLearningCard(
    status: String,
    title: String,
    description: String,
    time_remaining: String,
    lesson_progress: String,
    progress_percent: u8,
    cta_label: String,
    cta_href: String,
    #[prop(optional)] img_link: Option<String>,
) -> impl IntoView {
    let img_link = img_link.unwrap_or_else(|| {
        "https://images.unsplash.com/photo-1501504905252-473c47e087f8?w=128&h=128&fit=crop"
            .to_string()
    });

    view! {
        <article class="min-w-0 overflow-hidden rounded-2xl bg-linear-to-br from-purple-100 to-indigo-100 p-5 shadow-md sm:p-6 md:px-8 md:py-7">
            <div class="flex min-w-0 flex-col gap-6 lg:flex-row">
                <div class="w-full overflow-hidden rounded-xl lg:w-64 lg:shrink-0 lg:self-stretch">
                    <img
                        src = {img_link}
                        alt="Continue Learning"
                        class="h-48 w-full object-cover sm:h-56 lg:h-full lg:min-h-64"
                    />
                </div>
                <div class = "min-w-0 items-center justify-center gap-6 w-full flex flex-col sm:flex-row">
                    <div class="min-w-0 w-full flex-1">
                        <span class="mb-3 inline-flex rounded-full bg-purple-600 px-3 py-1 text-xs font-semibold uppercase text-white">
                            {status}
                        </span>

                        <h3 class="mb-2 min-w-0 text-xl font-bold text-foreground-900 sm:text-2xl text-balance break-words">
                            {title}
                        </h3>

                        <p class="mb-4 min-w-0 text-foreground-600 break-words">
                            {description}
                        </p>

                        <div class="mb-4 flex flex-wrap items-center gap-x-6 gap-y-2 text-sm text-foreground-600">
                            <div class="flex items-center gap-2">
                                <span aria-hidden="true">"⏱️"</span>
                                <span>{time_remaining}</span>
                            </div>

                            <div class="flex items-center gap-2">
                                <span aria-hidden="true">"✅"</span>
                                <span>{lesson_progress}</span>
                            </div>
                        </div>

                        <div class="mb-4">
                            <div class="mb-2 flex items-center justify-between">
                                <span class="text-sm font-medium text-foreground-600">"Progress"</span>
                                <span class="text-sm font-bold text-foreground-900">{progress_percent}"%"</span>
                            </div>

                            <div class="h-2.5 w-full overflow-hidden rounded-full bg-purple-200">
                                <div
                                    class="h-full rounded-full bg-linear-to-r from-purple-600 to-purple-500 transition-[width] duration-300 ease-out"
                                    style=format!("width: {}%;", progress_percent)
                                />
                            </div>
                        </div>
                    </div>

                    <div class="w-full shrink-0 sm:w-auto">
                        <A
                            href=cta_href
                            attr:class="inline-flex min-h-[44px] w-full items-center justify-center rounded-xl bg-indigo-400 px-8 py-3 font-semibold text-white shadow-lg transition hover:bg-indigo-500 hover:shadow-xl sm:w-auto xl:w-auto"
                        >
                            {cta_label}
                        </A>
                    </div>
                </div>
            </div>
        </article>
    }
}

#[component]
pub fn CourseCard(
    category: String,
    category_class: String,
    badge: String,
    badge_class: String,
    title: String,
    description: String,
    duration: String,
    lesson_count: String,
    #[prop(optional)] progress_percent: Option<u8>,
    #[prop(optional)] img_link: Option<String>,
    instructor_initials: String,
    instructor_name: String,
    cta_label: String,
    cta_href: String,
) -> impl IntoView {
    let image_alt = format!("{} course cover", title);
    let img_link = img_link.unwrap_or_else(|| {
        "https://images.unsplash.com/photo-1481627834876-b7833e8f5570?auto=format&fit=crop&w=640&h=360&q=80".to_string()
    });

    let progress_view = progress_percent.map(|progress_percent| {
        view! {
            <div class="mb-4">
                <div class="mb-2 flex items-center justify-between">
                    <span class="text-xs text-foreground-400">"Your Progress"</span>
                    <span class="text-xs font-semibold text-sky-700 dark:text-sky-400">{progress_percent}"%"</span>
                </div>

                <div class="h-1.5 w-full overflow-hidden rounded-full bg-gray-200">
                    <div
                        class="h-full rounded-full bg-purple-600 transition-[width] duration-300 ease-out"
                        style=format!("width: {}%;", progress_percent)
                    />
                </div>
            </div>
        }
    });

    view! {
        <article class="group w-full max-w-[19rem] shrink-0 snap-start overflow-hidden rounded-2xl bg-surface-700 p-5 shadow-md transition duration-300 ease-out hover:-translate-y-0.5 hover:shadow-[0_10px_25px_-5px_rgba(124,58,237,0.2)] sm:w-90 sm:max-w-none sm:p-6">
            <div class="mb-5 overflow-hidden rounded-xl bg-sky-100 dark:bg-sky-400/10">
                <img
                    src=img_link
                    alt=image_alt
                    class="h-40 w-full object-cover transition duration-300 ease-out group-hover:scale-105 md:h-50"
                />
            </div>

            <div class="mb-2 flex min-w-0 flex-wrap items-start justify-between gap-2 md:mb-4">
                <span class=format!("rounded-full px-3 py-1 text-xs font-semibold {}", category_class)>
                    {category}
                </span>

                <span class=format!("rounded-full px-3 py-1 text-xs font-semibold {}", badge_class)>
                    {badge}
                </span>
            </div>

            <h3 class="mb-2 min-w-0 text-lg font-bold text-foreground-900 sm:text-xl text-balance break-words">
                {title}
            </h3>

            <p class="mb-4 line-clamp-2 min-w-0 text-sm text-foreground-600 break-words">
                {description}
            </p>

            <div class="mb-4 flex flex-wrap items-center gap-x-4 gap-y-2 text-sm text-foreground-400">
                <div class="flex items-center gap-1">
                    <span aria-hidden="true">"⏱️"</span>
                    <span>{duration}</span>
                </div>

                <div class="flex items-center gap-1">
                    <span aria-hidden="true">"📖"</span>
                    <span>{lesson_count}</span>
                </div>
            </div>

            {progress_view}

            <div class="mb-4 flex min-w-0 items-center gap-2">
                <div class="flex h-8 w-8 shrink-0 items-center justify-center rounded-full bg-sky-200 dark:bg-sky-400/20 text-sm font-semibold text-sky-700 dark:text-sky-400">
                    {instructor_initials}
                </div>

                <span class="min-w-0 truncate text-sm text-foreground-600">
                    {instructor_name}
                </span>
            </div>

            <A
                href=cta_href
                attr:class="inline-flex min-h-[44px] w-full items-center justify-center rounded-xl bg-indigo-400 py-3 font-semibold text-white transition hover:bg-indigo-500"
            >
                {cta_label}
            </A>
        </article>
    }
}

#[component]
pub fn LearningPathCard(
    title: String,
    description: String,
    title_class: String,
    description_class: String,
    icon: String,
    icon_class: String,
    course_count: String,
    duration: String,
    progress_label: String,
    progress_percent: u8,
    cta_label: String,
    cta_href: String,
) -> impl IntoView {
    view! {
        <article class="group relative min-w-0 overflow-hidden rounded-2xl bg-surface-700 bg-[radial-gradient(circle_at_2px_2px,rgba(139,92,246,0.15)_1px,transparent_0)] bg-size-[32px_32px] bg-position-[0_0] p-5 shadow-lg transition-[background-color] duration-300 ease-out hover:bg-[#fbfaff] dark:hover:bg-[#272352] sm:p-6 md:p-8">
            <div class="relative mb-4 flex min-w-0 items-start justify-between gap-3">
                <div class="min-w-0">
                    <h3 class=format!("mb-2 min-w-0 text-xl font-bold sm:text-2xl text-balance break-words transition-colors duration-300 ease-out {}", title_class)>
                        {title}
                    </h3>

                    <p class=format!("mb-4 min-w-0 text-sm sm:text-base break-words transition-colors duration-300 ease-out {}", description_class)>
                        {description}
                    </p>
                </div>

                <div class="relative grid h-14 w-14 shrink-0 place-items-center rounded-2xl bg-white/70 dark:bg-white/10 ring-1 ring-stroke-muted" aria-hidden="true">
                    <div class=format!("relative text-4xl {}", icon_class)>
                        {icon}
                    </div>
                </div>
            </div>

            <div class="relative mb-6 flex flex-wrap items-center gap-3 text-foreground-400">
                <div class="flex items-center gap-2 rounded-full bg-white/50 dark:bg-white/10 px-3 py-1.5 text-sm">
                    <span aria-hidden="true">"📖"</span>
                    <span>{course_count}</span>
                </div>

                <div class="flex items-center gap-2 rounded-full bg-white/50 dark:bg-white/10 px-3 py-1.5 text-sm">
                    <span aria-hidden="true">"⏱️"</span>
                    <span>{duration}</span>
                </div>
            </div>

            <div class="relative mb-4">
                <div class="mb-2 flex items-center justify-between">
                    <span class="text-sm text-foreground-600">"Path Progress"</span>
                    <span class="text-sm font-bold text-foreground-600">{progress_label}</span>
                </div>

                <div class="h-2 w-full overflow-hidden rounded-full bg-purple-900/30 ring-1 ring-purple-900/0">
                    <div
                        class="relative h-full overflow-hidden rounded-full bg-purple-500 transition-[width] duration-300 ease-out"
                        style=format!("width: {}%;", progress_percent)
                    ></div>
                </div>
            </div>

            <A
                href=cta_href
                attr:class="relative flex min-h-[44px] w-full items-center justify-center gap-2 rounded-xl bg-indigo-400 py-3 font-semibold text-white transition-colors duration-300 ease-out hover:bg-indigo-500"
            >
                <span class="relative">{cta_label}</span>
                <span class="relative" aria-hidden="true">"→"</span>
            </A>
        </article>
    }
}
