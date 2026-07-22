use leptos::prelude::*;

#[component]
pub fn ContinueLearningCard(
    status: String,
    title: String,
    description: String,
    time_remaining: String,
    lesson_progress: String,
    progress_percent: u8,
    cta_label: String,
    #[prop(optional)] img_link: Option<String>,
) -> impl IntoView {
    let img_link = img_link.unwrap_or_else(|| {
        "https://images.unsplash.com/photo-1501504905252-473c47e087f8?w=128&h=128&fit=crop".to_string()
    });

    view! {
        <article class="rounded-2xl border border-purple-200 bg-linear-to-br from-purple-100 to-indigo-100 p-6 shadow-md md:p-8">
            <div class="flex gap-6 ">
                <div class="w-50">
                    <img src = {img_link} alt="Continue Learning" class="mb-4 w-full rounded-xl md:mb-6 xl:w-64" />
                </div>
                <div class = "items-center justify-center gap-6 w-full flex">
                    <div class="flex-1">
                        <span class="mb-3 inline-flex rounded-full bg-purple-600 px-3 py-1 text-xs font-semibold uppercase text-white">
                            {status}
                        </span>

                        <h3 class="mb-2 text-2xl font-bold text-gray-900">
                            {title}
                        </h3>

                        <p class="mb-4 text-gray-700">
                            {description}
                        </p>

                        <div class="mb-4 flex flex-wrap items-center gap-x-6 gap-y-2 text-sm text-gray-700">
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
                                <span class="text-sm font-medium text-gray-700">"Progress"</span>
                                <span class="text-sm font-bold text-gray-900">{progress_percent}"%"</span>
                            </div>

                            <div class="h-2.5 w-full overflow-hidden rounded-full bg-purple-200">
                                <div
                                    class="h-full rounded-full bg-linear-to-r from-purple-600 to-purple-500 transition-[width] duration-300 ease-out"
                                    style=format!("width: {}%;", progress_percent)
                                />
                            </div>
                        </div>
                    </div>

                    <div>
                        <button class="w-full rounded-xl bg-purple-600 px-8 py-4 font-semibold text-white shadow-lg transition hover:bg-purple-700 hover:shadow-xl xl:w-auto">
                            {cta_label}
                        </button>
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
) -> impl IntoView {
    let image_alt = format!("{} course cover", title);
    let img_link = img_link.unwrap_or_else(|| {
        "https://images.unsplash.com/photo-1481627834876-b7833e8f5570?auto=format&fit=crop&w=640&h=360&q=80".to_string()
    });

    let progress_view = progress_percent.map(|progress_percent| {
        view! {
            <div class="mb-4">
                <div class="mb-2 flex items-center justify-between">
                    <span class="text-xs text-gray-500">"Your Progress"</span>
                    <span class="text-xs font-semibold text-purple-700">{progress_percent}"%"</span>
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
        <article class="group rounded-2xl border border-purple-100 bg-white p-6 shadow-md transition duration-300 ease-out hover:-translate-y-0.5 hover:shadow-[0_10px_25px_-5px_rgba(124,58,237,0.2)]">
            <div class="mb-5 overflow-hidden rounded-xl bg-purple-100">
                <img
                    src=img_link
                    alt=image_alt
                    class="h-50 w-full object-cover transition duration-300 ease-out group-hover:scale-105"
                />
            </div>

            <div class="mb-4 flex items-start justify-between gap-3">
                <span class=format!("rounded-full px-3 py-1 text-xs font-semibold {}", category_class)>
                    {category}
                </span>

                <span class=format!("rounded-full px-3 py-1 text-xs font-semibold {}", badge_class)>
                    {badge}
                </span>
            </div>

            <h3 class="mb-2 text-xl font-bold text-purple-900">
                {title}
            </h3>

            <p class="mb-4 line-clamp-2 text-sm text-gray-600">
                {description}
            </p>

            <div class="mb-4 flex flex-wrap items-center gap-x-4 gap-y-2 text-sm text-gray-500">
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

            <div class="mb-4 flex items-center gap-2">
                <div class="flex h-8 w-8 items-center justify-center rounded-full bg-purple-200 text-sm font-semibold text-purple-700">
                    {instructor_initials}
                </div>

                <span class="text-sm text-gray-700">
                    {instructor_name}
                </span>
            </div>

            <button class="w-full rounded-xl bg-purple-600 py-3 font-semibold text-white transition hover:bg-purple-700">
                {cta_label}
            </button>
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
) -> impl IntoView {
    view! {
        <article class="rounded-2xl bg-white bg-[radial-gradient(circle_at_2px_2px,rgba(139,92,246,0.15)_1px,transparent_0)] bg-[length:32px_32px] p-6 shadow-lg md:p-8">
            <div class="mb-4 flex items-start justify-between gap-4">
                <div>
                    <h3 class=format!("mb-2 text-2xl font-bold {}", title_class)>
                        {title}
                    </h3>

                    <p class=format!("mb-4 {}", description_class)>
                        {description}
                    </p>
                </div>

                <div class=format!("text-4xl {}", icon_class) aria-hidden="true">
                    {icon}
                </div>
            </div>

            <div class="mb-6 flex flex-wrap items-center gap-x-6 gap-y-2 text-gray-400">
                <div class="flex items-center gap-2">
                    <span aria-hidden="true">"📖"</span>
                    <span>{course_count}</span>
                </div>

                <div class="flex items-center gap-2">
                    <span aria-hidden="true">"⏱️"</span>
                    <span>{duration}</span>
                </div>
            </div>

            <div class="mb-4">
                <div class="mb-2 flex items-center justify-between">
                    <span class="text-sm text-gray-700">"Path Progress"</span>
                    <span class="text-sm font-bold text-gray-700">{progress_label}</span>
                </div>

                <div class="h-2 w-full overflow-hidden rounded-full bg-purple-900/30">
                    <div
                        class="h-full rounded-full bg-purple-500 transition-[width] duration-300 ease-out"
                        style=format!("width: {}%;", progress_percent)
                    />
                </div>
            </div>

            <button class="w-full rounded-xl bg-purple-700 py-3 font-semibold text-white transition hover:bg-purple-600">
                {cta_label}
            </button>
        </article>
    }
}
