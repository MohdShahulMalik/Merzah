use leptos::prelude::*;
use leptos::reactive::spawn_local;
use leptos_router::components::A;
use leptos_router::hooks::use_params_map;

use crate::models::education::EnrollmentProgress;
use crate::server_functions::education::fetch_my_courses;
use crate::server_functions::education_roadmap::fetch_roadmap_detail;
use crate::server_functions::education_roadmap::start_roadmap;

const NOT_LOGGED_IN: &str = "You are not logged in";

fn path_progress(course_ids: &[String], enrolled: &[EnrollmentProgress]) -> Option<f32> {
    let matched = enrolled
        .iter()
        .filter(|row| course_ids.iter().any(|id| id == &row.course_id))
        .map(|row| row.progress_percent)
        .collect::<Vec<_>>();

    if matched.is_empty() {
        return None;
    }

    let total = matched.iter().sum::<f32>();

    Some((total / matched.len() as f32).clamp(0.0, 100.0))
}

#[component]
pub fn LearningPathDetailPage() -> impl IntoView {
    let params = use_params_map();
    let roadmap_id = move || params.with(|p| p.get("roadmap_id").unwrap_or_default());

    let roadmap = Resource::new(
        move || roadmap_id(),
        |roadmap_id: String| async move {
            if roadmap_id.is_empty() {
                return None;
            }

            fetch_roadmap_detail(roadmap_id)
                .await
                .ok()
                .and_then(|res| res.data)
        },
    );

    let my_courses = Resource::new(
        move || roadmap_id(),
        |_roadmap_id: String| async move {
            fetch_my_courses()
                .await
                .ok()
                .and_then(|res| res.data)
                .unwrap_or_default()
        },
    );

    let refetched_enrolled = RwSignal::new(None::<Vec<EnrollmentProgress>>);
    let starting = RwSignal::new(false);
    let start_error = RwSignal::new(None::<String>);

    let on_start = move |_| {
        let id = roadmap_id();

        if id.is_empty() || starting.get_untracked() {
            return;
        }

        starting.set(true);
        start_error.set(None);

        spawn_local(async move {
            match start_roadmap(id).await {
                Ok(response) => {
                    if let Some(error) = response.error {
                        start_error.set(Some(error));
                    } else {
                        match fetch_my_courses().await {
                            Ok(courses_response) => match courses_response.data {
                                Some(rows) => refetched_enrolled.set(Some(rows)),
                                None => {
                                    if let Some(error) = courses_response.error {
                                        start_error.set(Some(error));
                                    }
                                }
                            },
                            Err(e) => start_error.set(Some(e.to_string())),
                        }
                    }
                }
                Err(e) => start_error.set(Some(e.to_string())),
            }

            starting.set(false);
        });
    };

    let error_banner = move || {
        start_error.get().map(|message| {
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
                                    "Log in to start"
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
                        <div class="h-60 w-full rounded-3xl bg-surface-700 shadow-md sm:h-72"></div>
                        <div class="h-24 w-full rounded-2xl bg-surface-700"></div>
                        <div class="h-24 w-full rounded-2xl bg-surface-700"></div>
                        <p class="text-sm text-foreground-600">"Loading learning path…"</p>
                    </div>
                }
            }>
                {move || {
                    match roadmap.get().flatten() {
                        None => {
                            view! {
                                <div class="min-w-0 space-y-3 rounded-3xl bg-surface-700 p-8 text-center shadow-md">
                                    <p class="text-4xl" aria-hidden="true">"🧭"</p>
                                    <h1 class="text-xl font-bold text-foreground-900 sm:text-2xl">
                                        "Learning path not found."
                                    </h1>
                                    <p class="mx-auto max-w-sm text-sm text-foreground-600">
                                        "This path may have been unpublished or the link is out of date."
                                    </p>
                                    <A
                                        href="/learn"
                                        attr:class="inline-flex min-h-[44px] items-center justify-center gap-2 rounded-xl bg-indigo-400 px-6 font-semibold text-white transition duration-300 ease-out hover:bg-indigo-500 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-indigo-300"
                                    >
                                        "Browse learning paths"
                                        <span aria-hidden="true">"→"</span>
                                    </A>
                                </div>
                            }
                                .into_any()
                        }
                        Some(detail) => {
                            let courses = detail.courses.clone();
                            let course_total = courses.len();
                            let has_courses = course_total > 0;
                            let required_total = courses
                                .iter()
                                .filter(|course| course.is_required)
                                .count();
                            let course_ids = courses
                                .iter()
                                .map(|course| course.course_id.clone())
                                .collect::<Vec<_>>();
                            let enrolled_rows = refetched_enrolled
                                .get()
                                .or_else(|| my_courses.get())
                                .unwrap_or_default();
                            let percent = path_progress(&course_ids, &enrolled_rows);
                            let hero_image = detail.image_url.clone();

                            view! {
                                <article class="min-w-0 space-y-8">
                                    <section class="relative isolate min-w-0 overflow-hidden rounded-3xl bg-linear-to-br from-purple-600 via-indigo-500 to-sky-500 p-px shadow-[0_24px_60px_-30px_rgba(79,70,229,0.75)]">
                                        <div class="relative min-w-0 overflow-hidden rounded-3xl bg-surface-700/95 backdrop-blur-sm">
                                            <div
                                                class="pointer-events-none absolute -left-24 -top-24 h-64 w-64 rounded-full bg-indigo-500/25 blur-3xl"
                                                aria-hidden="true"
                                            ></div>
                                            <div
                                                class="pointer-events-none absolute -bottom-24 -right-16 h-56 w-56 rounded-full bg-purple-500/25 blur-3xl"
                                                aria-hidden="true"
                                            ></div>

                                            {hero_image
                                                .map(|image_url| {
                                                    view! {
                                                        <div class="relative min-w-0">
                                                            <img
                                                                src=image_url
                                                                alt=detail.title.clone()
                                                                class="h-44 w-full object-cover sm:h-56 lg:h-64"
                                                            />
                                                            <div
                                                                class="pointer-events-none absolute inset-0 bg-linear-to-t from-surface-700 via-surface-700/40 to-transparent"
                                                                aria-hidden="true"
                                                            ></div>
                                                        </div>
                                                    }
                                                })}

                                            <div class="relative min-w-0 space-y-4 p-5 sm:p-7 lg:p-9">
                                                <A
                                                    href="/learn"
                                                    attr:class="inline-flex min-h-[44px] items-center gap-2 text-sm font-medium text-sky-600 transition duration-300 ease-out hover:-translate-x-0.5 hover:text-sky-700 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-sky-400 dark:text-sky-400 dark:hover:text-sky-300"
                                                >
                                                    <span aria-hidden="true">"←"</span>
                                                    "Back to Learn"
                                                </A>

                                                <span class="inline-flex items-center gap-2 rounded-full bg-linear-to-r from-purple-600 to-indigo-500 px-3 py-1 text-xs font-semibold uppercase tracking-wide text-white shadow-sm">
                                                    <span aria-hidden="true">"🧭"</span>
                                                    "Guided path"
                                                </span>

                                                <h1 class="min-w-0 break-words text-3xl font-bold leading-tight text-foreground-900 text-balance sm:text-4xl">
                                                    {detail.title.clone()}
                                                </h1>

                                                <p class="min-w-0 max-w-3xl whitespace-pre-line break-words text-base text-foreground-600 sm:text-lg">
                                                    {detail.description.clone()}
                                                </p>

                                                <div class="flex min-w-0 flex-wrap items-center gap-2">
                                                    <span class="inline-flex items-center gap-2 rounded-full bg-purple-500/10 px-3 py-1.5 text-xs font-semibold text-purple-700 ring-1 ring-purple-500/20 dark:text-purple-300">
                                                        <span aria-hidden="true">"✦"</span>
                                                        {format!("{:?}", detail.difficulty)}
                                                    </span>
                                                    <span class="inline-flex items-center gap-2 rounded-full bg-black/5 px-3 py-1.5 text-xs font-medium text-foreground-600 ring-1 ring-black/5 dark:bg-white/10 dark:ring-white/10">
                                                        <span aria-hidden="true">"🗓️"</span>
                                                        {format!("{} weeks", detail.estimated_weeks)}
                                                    </span>
                                                    <span class="inline-flex items-center gap-2 rounded-full bg-black/5 px-3 py-1.5 text-xs font-medium text-foreground-600 ring-1 ring-black/5 dark:bg-white/10 dark:ring-white/10">
                                                        <span aria-hidden="true">"📚"</span>
                                                        {format!("{} courses", course_total)}
                                                    </span>
                                                    <span class="inline-flex items-center gap-2 rounded-full bg-black/5 px-3 py-1.5 text-xs font-medium text-foreground-600 ring-1 ring-black/5 dark:bg-white/10 dark:ring-white/10">
                                                        <span aria-hidden="true">"🎯"</span>
                                                        {format!("{} required", required_total)}
                                                    </span>
                                                </div>

                                                <div class="min-w-0 rounded-2xl bg-black/5 p-4 ring-1 ring-black/5 sm:p-5 dark:bg-white/5 dark:ring-white/10">
                                                    {match percent {
                                                        Some(value) => {
                                                            view! {
                                                                <div class="min-w-0 space-y-3">
                                                                    <div class="flex min-w-0 items-center justify-between gap-2">
                                                                        <span class="text-xs font-semibold uppercase tracking-wide text-foreground-600">
                                                                            "Path progress"
                                                                        </span>
                                                                        <span class="text-sm font-bold text-foreground-900">
                                                                            {format!("{value:.0}% of required courses")}
                                                                        </span>
                                                                    </div>

                                                                    <div class="h-2.5 w-full overflow-hidden rounded-full bg-purple-900/15 ring-1 ring-purple-500/10 dark:bg-white/10">
                                                                        <div
                                                                            class="h-full rounded-full bg-linear-to-r from-purple-600 via-indigo-500 to-sky-400 shadow-[0_0_12px_rgba(99,102,241,0.65)] transition-[width] duration-300 ease-out"
                                                                            style=format!("width: {value:.0}%;")
                                                                        ></div>
                                                                    </div>

                                                                    <p class="min-w-0 break-words text-sm text-foreground-600">
                                                                        "You're on this path — pick up any step below to keep moving."
                                                                    </p>
                                                                </div>
                                                            }
                                                                .into_any()
                                                        }
                                                        None => {
                                                            view! {
                                                                <div class="min-w-0 space-y-3">
                                                                    <p class="min-w-0 break-words text-sm text-foreground-600">
                                                                        "Begin this path to enroll in every required course at once and track one combined progress bar."
                                                                    </p>

                                                                    <button
                                                                        class="inline-flex min-h-[44px] w-full items-center justify-center gap-2 rounded-xl bg-indigo-400 px-5 font-semibold text-white shadow-lg transition duration-300 ease-out hover:-translate-y-0.5 hover:bg-indigo-500 hover:shadow-xl focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-indigo-300 disabled:translate-y-0 disabled:opacity-60 sm:w-auto"
                                                                        disabled=move || starting.get()
                                                                        on:click=on_start
                                                                    >
                                                                        "Begin Path"
                                                                        <span aria-hidden="true">"→"</span>
                                                                    </button>

                                                                    {move || {
                                                                        starting
                                                                            .get()
                                                                            .then(|| {
                                                                                view! {
                                                                                    <p class="text-sm font-medium text-foreground-600">
                                                                                        "Setting up your path…"
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
                                            </div>
                                        </div>
                                    </section>

                                    <section class="min-w-0 space-y-5">
                                        <div class="flex min-w-0 flex-wrap items-end justify-between gap-2">
                                            <h2 class="min-w-0 text-xl font-bold text-foreground-900 sm:text-2xl">
                                                "The Journey"
                                            </h2>
                                            <span class="text-xs font-medium uppercase tracking-wide text-foreground-600">
                                                "Follow the steps in order"
                                            </span>
                                        </div>

                                        {(!has_courses)
                                            .then(|| {
                                                view! {
                                                    <div class="min-w-0 rounded-2xl border border-dashed border-indigo-400/40 bg-surface-700 p-8 text-center shadow-sm">
                                                        <p class="text-3xl" aria-hidden="true">"🗺️"</p>
                                                        <p class="mt-2 text-sm font-semibold text-foreground-900">
                                                            "No courses in this path yet."
                                                        </p>
                                                        <p class="mt-1 text-sm text-foreground-600">
                                                            "This roadmap is still being charted — check back soon."
                                                        </p>
                                                    </div>
                                                }
                                            })}

                                        <ol class="relative min-w-0 space-y-4 sm:space-y-5">
                                            {courses
                                                .clone()
                                                .into_iter()
                                                .enumerate()
                                                .map(|(index, course)| {
                                                    let is_last = index + 1 == course_total;
                                                    let is_required = course.is_required;
                                                    let node_class = if is_required {
                                                        "absolute left-0 top-6 hidden h-11 w-11 place-items-center rounded-full bg-linear-to-br from-purple-600 to-indigo-500 text-sm font-bold text-white shadow-lg ring-4 ring-surface-700 sm:grid"
                                                    } else {
                                                        "absolute left-0 top-6 hidden h-11 w-11 place-items-center rounded-full border-2 border-dashed border-indigo-400/60 bg-surface-700 text-sm font-bold text-foreground-600 ring-4 ring-surface-700 sm:grid"
                                                    };
                                                    let card_class = if is_required {
                                                        "group block min-w-0 rounded-2xl bg-surface-700 p-5 shadow-md ring-1 ring-indigo-400/20 transition duration-300 ease-out hover:-translate-y-0.5 hover:shadow-[0_20px_45px_-28px_rgba(99,102,241,0.95)] hover:ring-indigo-400/50 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-indigo-400 sm:p-6"
                                                    } else {
                                                        "group block min-w-0 rounded-2xl border border-dashed border-stroke-muted bg-surface-700/80 p-5 shadow-sm transition duration-300 ease-out hover:-translate-y-0.5 hover:border-indigo-400/50 hover:shadow-md focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-indigo-400 sm:p-6"
                                                    };
                                                    let pill_class = if is_required {
                                                        "inline-flex shrink-0 items-center gap-1 rounded-full bg-indigo-400/15 px-3 py-1 text-xs font-semibold text-indigo-700 dark:text-indigo-300"
                                                    } else {
                                                        "inline-flex shrink-0 items-center gap-1 rounded-full border border-stroke-muted px-3 py-1 text-xs font-semibold text-foreground-600"
                                                    };

                                                    view! {
                                                        <li class="relative min-w-0 sm:pl-16">
                                                            <span class=node_class aria-hidden="true">
                                                                {format!("{:02}", index + 1)}
                                                            </span>

                                                            {(!is_last)
                                                                .then(|| {
                                                                    view! {
                                                                        <span
                                                                            class="absolute bottom-[-1.25rem] left-[1.3125rem] top-[4.25rem] hidden w-px bg-linear-to-b from-indigo-400/70 to-transparent sm:block"
                                                                            aria-hidden="true"
                                                                        ></span>
                                                                    }
                                                                })}

                                                            <A
                                                                href=format!("/learn/courses/{}", course.course_id)
                                                                attr:class=card_class
                                                            >
                                                                <div class="flex min-w-0 flex-wrap items-start justify-between gap-2">
                                                                    <h3 class="min-w-0 break-words text-base font-bold text-foreground-900 transition duration-300 ease-out group-hover:text-indigo-600 sm:text-lg dark:group-hover:text-indigo-300">
                                                                        {course.title.clone()}
                                                                    </h3>
                                                                    <span class=pill_class>
                                                                        {if is_required { "Required" } else { "Optional" }}
                                                                    </span>
                                                                </div>

                                                                <p class="mt-2 line-clamp-3 min-w-0 break-words text-sm text-foreground-600">
                                                                    {course.short_description.clone()}
                                                                </p>

                                                                {course
                                                                    .note
                                                                    .clone()
                                                                    .map(|note| {
                                                                        view! {
                                                                            <p class="mt-3 flex min-w-0 items-start gap-2 rounded-xl bg-purple-500/10 px-3 py-2 text-xs font-medium text-purple-700 dark:text-purple-300">
                                                                                <span class="shrink-0" aria-hidden="true">"💡"</span>
                                                                                <span class="min-w-0 break-words">{note}</span>
                                                                            </p>
                                                                        }
                                                                    })}

                                                                <span class="mt-4 inline-flex min-h-[44px] items-center gap-2 text-sm font-semibold text-sky-600 transition duration-300 ease-out group-hover:gap-3 dark:text-sky-400">
                                                                    "Open course"
                                                                    <span aria-hidden="true">"→"</span>
                                                                </span>
                                                            </A>
                                                        </li>
                                                    }
                                                })
                                                .collect_view()}
                                        </ol>
                                    </section>
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
