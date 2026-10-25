use leptos::prelude::*;
use leptos::reactive::spawn_local;
use leptos_router::components::A;
use leptos_router::hooks::use_params_map;

use crate::models::education::LessonContentType;
use crate::server_functions::education::enroll_course;
use crate::server_functions::education::fetch_course_details;
use crate::server_functions::education::fetch_course_progress;
use crate::server_functions::education::fetch_my_courses;

const FALLBACK_THUMBNAIL: &str = "https://images.unsplash.com/photo-1481627834876-b7833e8f5570?auto=format&fit=crop&w=1280&h=560&q=80";
const NOT_LOGGED_IN: &str = "You are not logged in";

fn format_duration(total_minutes: i32) -> String {
    let minutes = total_minutes.max(0);
    let hours = minutes / 60;
    let remainder = minutes % 60;

    if hours > 0 {
        format!("{hours}h {remainder}m")
    } else {
        format!("{remainder}m")
    }
}

fn content_type_icon(content_type: &LessonContentType) -> &'static str {
    match content_type {
        LessonContentType::Text => "📄",
        LessonContentType::Video => "🎬",
        LessonContentType::Audio => "🎧",
        LessonContentType::Pdf => "📕",
        LessonContentType::ExternalLink => "🔗",
        LessonContentType::Mixed => "🧩",
    }
}

fn content_type_chip(content_type: &LessonContentType) -> &'static str {
    match content_type {
        LessonContentType::Text => "bg-sky-500/10 text-sky-700 dark:text-sky-300",
        LessonContentType::Video => "bg-purple-500/10 text-purple-700 dark:text-purple-300",
        LessonContentType::Audio => "bg-indigo-500/10 text-indigo-700 dark:text-indigo-300",
        LessonContentType::Pdf => "bg-rose-500/10 text-rose-700 dark:text-rose-300",
        LessonContentType::ExternalLink => "bg-emerald-500/10 text-emerald-700 dark:text-emerald-300",
        LessonContentType::Mixed => "bg-amber-500/10 text-amber-700 dark:text-amber-300",
    }
}

fn initials(name: &str) -> String {
    let mut out = String::new();

    for word in name.split_whitespace().take(2) {
        if let Some(first) = word.chars().next() {
            out.extend(first.to_uppercase());
        }
    }

    if out.is_empty() {
        out.push('?');
    }

    out
}

#[component]
pub fn CourseDetailPage() -> impl IntoView {
    let params = use_params_map();
    let course_id = move || params.with(|p| p.get("course_id").unwrap_or_default());

    let course = Resource::new(
        move || course_id(),
        |course_id: String| async move {
            if course_id.is_empty() {
                return None;
            }

            fetch_course_details(course_id)
                .await
                .ok()
                .and_then(|res| res.data)
        },
    );

    let my_courses = Resource::new(
        move || course_id(),
        |_course_id: String| async move {
            fetch_my_courses()
                .await
                .ok()
                .and_then(|res| res.data)
                .unwrap_or_default()
        },
    );

    let local_progress = RwSignal::new(None::<f32>);
    let enrolling = RwSignal::new(false);
    let enroll_error = RwSignal::new(None::<String>);

    let on_enroll = move |_| {
        let id = course_id();

        if id.is_empty() || enrolling.get_untracked() {
            return;
        }

        enrolling.set(true);
        enroll_error.set(None);

        spawn_local(async move {
            match enroll_course(id.clone()).await {
                Ok(response) => {
                    if let Some(error) = response.error {
                        enroll_error.set(Some(error));
                    } else {
                        match fetch_course_progress(id).await {
                            Ok(progress_response) => match progress_response.data {
                                Some(progress) => local_progress.set(Some(progress.progress_percent)),
                                None => {
                                    if let Some(error) = progress_response.error {
                                        enroll_error.set(Some(error));
                                    } else {
                                        local_progress.set(Some(0.0));
                                    }
                                }
                            },
                            Err(e) => enroll_error.set(Some(e.to_string())),
                        }
                    }
                }
                Err(e) => enroll_error.set(Some(e.to_string())),
            }

            enrolling.set(false);
        });
    };

    let error_banner = move || {
        enroll_error.get().map(|message| {
            let needs_login = message == NOT_LOGGED_IN;

            view! {
                <div class="min-w-0 space-y-2 rounded-xl bg-rose-500/10 p-4 ring-1 ring-rose-500/20">
                    <p class="min-w-0 break-words text-sm font-medium text-rose-600 dark:text-rose-400">
                        {message}
                    </p>

                    {needs_login
                        .then(|| {
                            view! {
                                <A
                                    href="/login"
                                    attr:class="inline-flex min-h-[44px] items-center gap-1 text-sm font-semibold text-sky-600 underline-offset-4 transition duration-300 ease-out hover:text-sky-700 hover:underline focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-sky-400 dark:text-sky-400 dark:hover:text-sky-300"
                                >
                                    "Log in to enroll"
                                    <span aria-hidden="true">"→"</span>
                                </A>
                            }
                        })}
                </div>
            }
        })
    };

    view! {
        <div class="min-w-0 space-y-8 py-6 sm:space-y-10 sm:py-8">
            <Suspense fallback=move || {
                view! {
                    <div class="min-w-0 animate-pulse space-y-6">
                        <div class="h-64 w-full rounded-3xl bg-surface-700 shadow-md sm:h-80"></div>
                        <div class="grid min-w-0 gap-6 lg:grid-cols-[minmax(0,1fr)_21rem]">
                            <div class="space-y-4">
                                <div class="h-28 w-full rounded-2xl bg-surface-700"></div>
                                <div class="h-40 w-full rounded-2xl bg-surface-700"></div>
                            </div>
                            <div class="h-56 w-full rounded-2xl bg-surface-700"></div>
                        </div>
                        <p class="text-sm text-foreground-600">"Loading course…"</p>
                    </div>
                }
            }>
                {move || {
                    match course.get().flatten() {
                        None => {
                            view! {
                                <div class="min-w-0 space-y-3 rounded-3xl bg-surface-700 p-8 text-center shadow-md">
                                    <p class="text-4xl" aria-hidden="true">"🧐"</p>
                                    <h1 class="text-xl font-bold text-foreground-900 sm:text-2xl">
                                        "Course not found."
                                    </h1>
                                    <p class="mx-auto max-w-sm text-sm text-foreground-600">
                                        "This course may have been unpublished or the link is out of date."
                                    </p>
                                    <A
                                        href="/learn"
                                        attr:class="inline-flex min-h-[44px] items-center justify-center gap-2 rounded-xl bg-indigo-400 px-6 font-semibold text-white transition duration-300 ease-out hover:bg-indigo-500 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-indigo-300"
                                    >
                                        "Browse courses"
                                        <span aria-hidden="true">"→"</span>
                                    </A>
                                </div>
                            }
                                .into_any()
                        }
                        Some(detail) => {
                            let thumbnail = detail
                                .thumbnail_url
                                .clone()
                                .unwrap_or_else(|| FALLBACK_THUMBNAIL.to_string());
                            let educator_name = detail.educator.display_name.clone();
                            let educator_initials = initials(&educator_name);
                            let modules = detail.modules.clone();
                            let module_count = modules.len();
                            let has_modules = module_count > 0;
                            let detail_id = detail.id.clone();
                            let percent = local_progress
                                .get()
                                .or_else(|| {
                                    my_courses
                                        .get()
                                        .unwrap_or_default()
                                        .iter()
                                        .find(|row| row.course_id == detail_id)
                                        .map(|row| row.progress_percent)
                                })
                                .map(|value| value.clamp(0.0, 100.0));

                            view! {
                                <article class="min-w-0 space-y-8">
                                    <section class="relative isolate min-w-0 overflow-hidden rounded-3xl bg-linear-to-br from-purple-600 via-indigo-500 to-sky-500 p-px shadow-[0_24px_60px_-30px_rgba(79,70,229,0.75)]">
                                        <div class="relative min-w-0 overflow-hidden rounded-3xl bg-surface-700/95 backdrop-blur-sm">
                                            <div
                                                class="pointer-events-none absolute -right-24 -top-24 h-64 w-64 rounded-full bg-purple-500/25 blur-3xl"
                                                aria-hidden="true"
                                            ></div>
                                            <div
                                                class="pointer-events-none absolute -bottom-28 -left-20 h-64 w-64 rounded-full bg-sky-400/20 blur-3xl"
                                                aria-hidden="true"
                                            ></div>

                                            <div class="relative min-w-0 p-5 sm:p-7 lg:p-9">
                                                <A
                                                    href="/learn"
                                                    attr:class="inline-flex min-h-[44px] items-center gap-2 text-sm font-medium text-sky-600 transition duration-300 ease-out hover:-translate-x-0.5 hover:text-sky-700 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-sky-400 dark:text-sky-400 dark:hover:text-sky-300"
                                                >
                                                    <span aria-hidden="true">"←"</span>
                                                    "Back to Learn"
                                                </A>

                                                <div class="mt-4 grid min-w-0 gap-6 lg:grid-cols-[minmax(0,1fr)_22rem] lg:items-center">
                                                    <div class="min-w-0 space-y-4">
                                                        <span class="inline-flex items-center gap-2 rounded-full bg-linear-to-r from-purple-600 to-indigo-500 px-3 py-1 text-xs font-semibold uppercase tracking-wide text-white shadow-sm">
                                                            <span aria-hidden="true">"✦"</span>
                                                            {format!("{:?}", detail.level)}
                                                        </span>

                                                        <h1 class="min-w-0 break-words text-3xl font-bold leading-tight text-foreground-900 text-balance sm:text-4xl">
                                                            {detail.title.clone()}
                                                        </h1>

                                                        <p class="min-w-0 max-w-2xl break-words text-base text-foreground-600 sm:text-lg">
                                                            {detail.short_description.clone()}
                                                        </p>

                                                        <div class="flex min-w-0 flex-wrap items-center gap-2">
                                                            <span class="inline-flex items-center gap-2 rounded-full bg-black/5 px-3 py-1.5 text-xs font-medium text-foreground-600 ring-1 ring-black/5 dark:bg-white/10 dark:ring-white/10">
                                                                <span aria-hidden="true">"⏱️"</span>
                                                                {format_duration(detail.duration_minutes)}
                                                            </span>
                                                            <span class="inline-flex items-center gap-2 rounded-full bg-black/5 px-3 py-1.5 text-xs font-medium text-foreground-600 ring-1 ring-black/5 dark:bg-white/10 dark:ring-white/10">
                                                                <span aria-hidden="true">"📖"</span>
                                                                {format!("{} lessons", detail.lesson_count)}
                                                            </span>
                                                            <span class="inline-flex items-center gap-2 rounded-full bg-black/5 px-3 py-1.5 text-xs font-medium text-foreground-600 ring-1 ring-black/5 dark:bg-white/10 dark:ring-white/10">
                                                                <span aria-hidden="true">"🗂️"</span>
                                                                {format!("{} modules", module_count)}
                                                            </span>
                                                            <span class="inline-flex items-center gap-2 rounded-full bg-black/5 px-3 py-1.5 text-xs font-medium text-foreground-600 ring-1 ring-black/5 dark:bg-white/10 dark:ring-white/10">
                                                                <span aria-hidden="true">"👥"</span>
                                                                {format!("{} enrolled", detail.enrollment_count)}
                                                            </span>
                                                        </div>

                                                        <div class="flex min-w-0 items-center gap-3 pt-1">
                                                            <span
                                                                class="grid h-11 w-11 shrink-0 place-items-center rounded-full bg-linear-to-br from-purple-500 to-indigo-500 text-sm font-bold text-white shadow-md"
                                                                aria-hidden="true"
                                                            >
                                                                {educator_initials}
                                                            </span>
                                                            <span class="min-w-0">
                                                                <span class="block text-xs uppercase tracking-wide text-foreground-600">
                                                                    "Educator"
                                                                </span>
                                                                <span class="block min-w-0 break-words text-sm font-semibold text-foreground-900">
                                                                    {educator_name}
                                                                </span>
                                                            </span>
                                                        </div>
                                                    </div>

                                                    <div class="relative min-w-0 overflow-hidden rounded-2xl ring-1 ring-black/5 dark:ring-white/10">
                                                        <img
                                                            src=thumbnail
                                                            alt=detail.title.clone()
                                                            class="h-52 w-full object-cover transition duration-300 ease-out hover:scale-[1.03] sm:h-64 lg:h-56"
                                                        />
                                                        <div
                                                            class="pointer-events-none absolute inset-0 bg-linear-to-t from-black/45 via-transparent to-transparent"
                                                            aria-hidden="true"
                                                        ></div>
                                                    </div>
                                                </div>
                                            </div>
                                        </div>
                                    </section>

                                    <div class="grid min-w-0 gap-6 lg:grid-cols-[minmax(0,1fr)_21rem] lg:items-start">
                                        <div class="min-w-0 space-y-6">
                                            <section class="min-w-0 rounded-2xl bg-surface-700 p-5 shadow-md sm:p-6">
                                                <h2 class="mb-3 text-lg font-bold text-foreground-900 sm:text-xl">
                                                    "About this course"
                                                </h2>
                                                <p class="min-w-0 whitespace-pre-line break-words text-sm leading-relaxed text-foreground-600 sm:text-base">
                                                    {detail.description.clone()}
                                                </p>
                                            </section>

                                            <section class="min-w-0 space-y-5">
                                                <div class="flex min-w-0 flex-wrap items-end justify-between gap-2">
                                                    <h2 class="min-w-0 text-xl font-bold text-foreground-900 sm:text-2xl">
                                                        "Course Content"
                                                    </h2>
                                                    <span class="text-xs font-medium uppercase tracking-wide text-foreground-600">
                                                        {format!(
                                                            "{} modules · {} lessons",
                                                            module_count,
                                                            detail.lesson_count,
                                                        )}
                                                    </span>
                                                </div>

                                                {(!has_modules)
                                                    .then(|| {
                                                        view! {
                                                            <div class="min-w-0 rounded-2xl border border-dashed border-indigo-400/40 bg-surface-700 p-8 text-center shadow-sm">
                                                                <p class="text-3xl" aria-hidden="true">"🪴"</p>
                                                                <p class="mt-2 text-sm font-semibold text-foreground-900">
                                                                    "No modules published yet."
                                                                </p>
                                                                <p class="mt-1 text-sm text-foreground-600">
                                                                    "The educator is still shaping this curriculum — check back soon."
                                                                </p>
                                                            </div>
                                                        }
                                                    })}

                                                <ol class="min-w-0 space-y-5">
                                                    {modules
                                                        .clone()
                                                        .into_iter()
                                                        .enumerate()
                                                        .map(|(index, module)| {
                                                            let is_last = index + 1 == module_count;
                                                            let lesson_total = module.lessons.len();

                                                            view! {
                                                                <li class="relative min-w-0 sm:pl-16">
                                                                    <span
                                                                        class="absolute left-0 top-5 hidden h-11 w-11 place-items-center rounded-2xl bg-linear-to-br from-purple-600 to-indigo-500 text-sm font-bold text-white shadow-lg ring-4 ring-surface-700 sm:grid"
                                                                        aria-hidden="true"
                                                                    >
                                                                        {format!("{:02}", index + 1)}
                                                                    </span>

                                                                    {(!is_last)
                                                                        .then(|| {
                                                                            view! {
                                                                                <span
                                                                                    class="absolute bottom-[-1.25rem] left-[1.3125rem] top-16 hidden w-px bg-linear-to-b from-indigo-400/70 to-transparent sm:block"
                                                                                    aria-hidden="true"
                                                                                ></span>
                                                                            }
                                                                        })}

                                                                    <div class="min-w-0 rounded-2xl bg-surface-700 p-5 shadow-md transition duration-300 ease-out hover:shadow-[0_18px_40px_-28px_rgba(99,102,241,0.9)] sm:p-6">
                                                                        <div class="flex min-w-0 flex-wrap items-center justify-between gap-2">
                                                                            <h3 class="min-w-0 break-words text-base font-bold text-foreground-900 sm:text-lg">
                                                                                {module.title.clone()}
                                                                            </h3>
                                                                            <span class="inline-flex shrink-0 items-center gap-1 rounded-full bg-indigo-400/10 px-3 py-1 text-xs font-semibold text-indigo-700 dark:text-indigo-300">
                                                                                {format!("{} lessons", lesson_total)}
                                                                            </span>
                                                                        </div>

                                                                        {module
                                                                            .description
                                                                            .clone()
                                                                            .map(|description| {
                                                                                view! {
                                                                                    <p class="mt-2 min-w-0 break-words text-sm text-foreground-600">
                                                                                        {description}
                                                                                    </p>
                                                                                }
                                                                            })}

                                                                        <ul class="mt-4 min-w-0 space-y-2">
                                                                            {module
                                                                                .lessons
                                                                                .clone()
                                                                                .into_iter()
                                                                                .map(|lesson| {
                                                                                    let is_preview = lesson.is_preview;

                                                                                    view! {
                                                                                        <li class="min-w-0">
                                                                                            <A
                                                                                                href=format!("/learn/lessons/{}", lesson.id)
                                                                                                attr:class="group flex min-h-[44px] min-w-0 items-center gap-3 rounded-xl bg-black/5 px-3 py-2.5 transition duration-300 ease-out hover:translate-x-1 hover:bg-indigo-400/10 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-indigo-400 dark:bg-white/5 dark:hover:bg-indigo-400/15"
                                                                                            >
                                                                                                <span
                                                                                                    class=format!(
                                                                                                        "grid h-9 w-9 shrink-0 place-items-center rounded-xl text-base {}",
                                                                                                        content_type_chip(&lesson.content_type),
                                                                                                    )
                                                                                                    aria-hidden="true"
                                                                                                >
                                                                                                    {content_type_icon(&lesson.content_type)}
                                                                                                </span>

                                                                                                <span class="min-w-0 flex-1">
                                                                                                    <span class="block min-w-0 break-words text-sm font-medium text-foreground-900">
                                                                                                        {lesson.title.clone()}
                                                                                                    </span>
                                                                                                    <span class="mt-0.5 flex flex-wrap items-center gap-2 text-xs text-foreground-600">
                                                                                                        <span>{format!("{} min", lesson.duration_minutes)}</span>
                                                                                                        {is_preview
                                                                                                            .then(|| {
                                                                                                                view! {
                                                                                                                    <span class="rounded-full bg-emerald-500/10 px-2 py-0.5 font-semibold text-emerald-700 dark:text-emerald-300">
                                                                                                                        "Free preview"
                                                                                                                    </span>
                                                                                                                }
                                                                                                            })}
                                                                                                    </span>
                                                                                                </span>

                                                                                                <span
                                                                                                    class="shrink-0 text-foreground-600 transition duration-300 ease-out group-hover:translate-x-1 group-hover:text-indigo-500"
                                                                                                    aria-hidden="true"
                                                                                                >
                                                                                                    "→"
                                                                                                </span>
                                                                                            </A>
                                                                                        </li>
                                                                                    }
                                                                                })
                                                                                .collect_view()}
                                                                        </ul>
                                                                    </div>
                                                                </li>
                                                            }
                                                        })
                                                        .collect_view()}
                                                </ol>
                                            </section>
                                        </div>

                                        <aside class="min-w-0 lg:sticky lg:top-6">
                                            <div class="min-w-0 overflow-hidden rounded-2xl bg-surface-700 p-5 shadow-lg ring-1 ring-black/5 sm:p-6 dark:ring-white/10">
                                                {match percent {
                                                    Some(value) if value >= 100.0 => {
                                                        view! {
                                                            <div class="min-w-0 space-y-3 text-center">
                                                                <span
                                                                    class="mx-auto grid h-16 w-16 place-items-center rounded-full bg-emerald-500/10 text-3xl"
                                                                    aria-hidden="true"
                                                                >
                                                                    "🎓"
                                                                </span>
                                                                <p class="text-lg font-bold text-emerald-700 dark:text-emerald-300">
                                                                    "Course completed"
                                                                </p>
                                                                <p class="min-w-0 break-words text-sm text-foreground-600">
                                                                    "Every lesson is done — revisit any module below whenever you want to refresh."
                                                                </p>
                                                                <div class="h-2 w-full overflow-hidden rounded-full bg-emerald-500/15">
                                                                    <div class="h-full w-full rounded-full bg-linear-to-r from-emerald-500 to-emerald-400"></div>
                                                                </div>
                                                            </div>
                                                        }
                                                            .into_any()
                                                    }
                                                    Some(value) => {
                                                        view! {
                                                            <div class="min-w-0 space-y-4">
                                                                <div class="flex min-w-0 items-center justify-between gap-2">
                                                                    <span class="text-xs font-semibold uppercase tracking-wide text-foreground-600">
                                                                        "Your progress"
                                                                    </span>
                                                                    <span class="text-sm font-bold text-foreground-900">
                                                                        {format!("{value:.0}% complete")}
                                                                    </span>
                                                                </div>

                                                                <div class="h-2.5 w-full overflow-hidden rounded-full bg-purple-900/15 ring-1 ring-purple-500/10 dark:bg-white/10">
                                                                    <div
                                                                        class="h-full rounded-full bg-linear-to-r from-purple-600 via-indigo-500 to-sky-400 shadow-[0_0_12px_rgba(99,102,241,0.65)] transition-[width] duration-300 ease-out"
                                                                        style=format!("width: {value:.0}%;")
                                                                    ></div>
                                                                </div>

                                                                <p class="min-w-0 break-words text-sm text-foreground-600">
                                                                    "You're enrolled — continue with the modules below."
                                                                </p>
                                                            </div>
                                                        }
                                                            .into_any()
                                                    }
                                                    None => {
                                                        view! {
                                                            <div class="min-w-0 space-y-4">
                                                                <div class="min-w-0 space-y-1">
                                                                    <p class="text-xs font-semibold uppercase tracking-wide text-foreground-600">
                                                                        "Ready when you are"
                                                                    </p>
                                                                    <p class="min-w-0 break-words text-sm text-foreground-600">
                                                                        "Enroll to unlock progress tracking across every module of this course."
                                                                    </p>
                                                                </div>

                                                                <button
                                                                    class="inline-flex min-h-[44px] w-full items-center justify-center gap-2 rounded-xl bg-indigo-400 px-5 font-semibold text-white shadow-lg transition duration-300 ease-out hover:-translate-y-0.5 hover:bg-indigo-500 hover:shadow-xl focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-indigo-300 disabled:translate-y-0 disabled:opacity-60"
                                                                    disabled=move || enrolling.get()
                                                                    on:click=on_enroll
                                                                >
                                                                    "Start Course"
                                                                    <span aria-hidden="true">"→"</span>
                                                                </button>

                                                                {move || {
                                                                    enrolling
                                                                        .get()
                                                                        .then(|| {
                                                                            view! {
                                                                                <p class="text-sm font-medium text-foreground-600">
                                                                                    "Enrolling…"
                                                                                </p>
                                                                            }
                                                                        })
                                                                }}

                                                                {error_banner}
                                                            </div>
                                                        }
                                                            .into_any()
                                                    }
                                                }}
                                            </div>
                                        </aside>
                                    </div>
                                </article>
                            }
                                .into_any()
                        }
                    }
                }}
            </Suspense>
        </div>
    }
}
