use leptos::prelude::*;
use leptos::reactive::spawn_local;
use leptos_router::components::A;
use leptos_router::hooks::use_params_map;

use crate::models::education::LessonContentType;
use crate::server_functions::education::complete_lesson;
use crate::server_functions::education::fetch_lesson_details;
use crate::server_functions::education::fetch_my_courses;

const NOT_LOGGED_IN: &str = "You are not logged in";

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

fn format_clock(total_seconds: i32) -> String {
    let seconds = total_seconds.max(0);

    format!("{}:{:02}", seconds / 60, seconds % 60)
}

#[component]
pub fn LessonPage() -> impl IntoView {
    let params = use_params_map();
    let lesson_id = move || params.with(|p| p.get("lesson_id").unwrap_or_default());

    let lesson = Resource::new(
        move || lesson_id(),
        |lesson_id: String| async move {
            if lesson_id.is_empty() {
                return None;
            }

            fetch_lesson_details(lesson_id)
                .await
                .ok()
                .and_then(|res| res.data)
        },
    );

    let my_courses = Resource::new(
        move || lesson_id(),
        |_lesson_id: String| async move {
            fetch_my_courses()
                .await
                .ok()
                .and_then(|res| res.data)
                .unwrap_or_default()
        },
    );

    let completing = RwSignal::new(false);
    let complete_error = RwSignal::new(None::<String>);
    let completed = RwSignal::new(false);

    let on_complete = move |_| {
        let id = lesson_id();

        if id.is_empty() || completing.get_untracked() {
            return;
        }

        completing.set(true);
        complete_error.set(None);

        spawn_local(async move {
            match complete_lesson(id).await {
                Ok(response) => {
                    if let Some(error) = response.error {
                        complete_error.set(Some(error));
                    } else {
                        completed.set(true);
                    }
                }
                Err(e) => complete_error.set(Some(e.to_string())),
            }

            completing.set(false);
        });
    };

    let error_banner = move || {
        complete_error.get().map(|message| {
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
                                    "Log in"
                                    <span aria-hidden="true">"→"</span>
                                </A>
                            }
                        })}
                </div>
            }
        })
    };

    view! {
        <div class="min-w-0 space-y-6 py-6 sm:space-y-8 sm:py-8">
            <Suspense fallback=move || {
                view! {
                    <div class="min-w-0 animate-pulse space-y-6">
                        <div class="h-10 w-56 rounded-xl bg-surface-700"></div>
                        <div class="h-64 w-full rounded-3xl bg-surface-700 shadow-md sm:h-80"></div>
                        <div class="h-40 w-full rounded-2xl bg-surface-700"></div>
                        <p class="text-sm text-foreground-600">"Loading lesson…"</p>
                    </div>
                }
            }>
                {move || {
                    match lesson.get().flatten() {
                        None => {
                            view! {
                                <div class="min-w-0 space-y-3 rounded-3xl bg-surface-700 p-8 text-center shadow-md">
                                    <p class="text-4xl" aria-hidden="true">"📭"</p>
                                    <h1 class="text-xl font-bold text-foreground-900 sm:text-2xl">
                                        "Lesson not found."
                                    </h1>
                                    <p class="mx-auto max-w-sm text-sm text-foreground-600">
                                        "This lesson may have been removed or the link is out of date."
                                    </p>
                                    <A
                                        href="/learn"
                                        attr:class="inline-flex min-h-[44px] items-center justify-center gap-2 rounded-xl bg-indigo-400 px-6 font-semibold text-white transition duration-300 ease-out hover:bg-indigo-500 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-indigo-300"
                                    >
                                        "Back to Learn"
                                        <span aria-hidden="true">"→"</span>
                                    </A>
                                </div>
                            }
                                .into_any()
                        }
                        Some(detail) => {
                            let course_href = format!("/learn/courses/{}", detail.course_id);
                            let enroll_href = course_href.clone();
                            let back_label = format!("Back to {}", detail.course_title);
                            let is_enrolled = my_courses
                                .get()
                                .unwrap_or_default()
                                .iter()
                                .any(|row| row.course_id == detail.course_id);
                            let video_url = detail.video_url.clone();
                            let audio_url = detail.audio_url.clone();
                            let thumbnail_url = detail.thumbnail_url.clone();
                            let show_audio = video_url.is_none();
                            let show_thumbnail = video_url.is_none();
                            let video_clock = detail.video_duration_seconds.map(format_clock);
                            let prev_lesson_id = detail.prev_lesson_id.clone();
                            let next_lesson_id = detail.next_lesson_id.clone();

                            view! {
                                <article class="min-w-0 space-y-6">
                                    <header class="min-w-0 space-y-4">
                                        <A
                                            href=course_href
                                            attr:class="inline-flex min-h-[44px] min-w-0 items-center gap-2 break-words text-sm font-medium text-sky-600 transition duration-300 ease-out hover:-translate-x-0.5 hover:text-sky-700 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-sky-400 dark:text-sky-400 dark:hover:text-sky-300"
                                        >
                                            <span aria-hidden="true">"←"</span>
                                            {back_label}
                                        </A>

                                        <div class="min-w-0 space-y-2">
                                            <p class="min-w-0 break-words text-xs font-semibold uppercase tracking-[0.18em] text-indigo-600 dark:text-indigo-300">
                                                {detail.module_title.clone()}
                                            </p>

                                            <h1 class="min-w-0 break-words text-3xl font-bold leading-tight text-foreground-900 text-balance sm:text-4xl">
                                                {detail.title.clone()}
                                            </h1>
                                        </div>

                                        <div class="flex min-w-0 flex-wrap items-center gap-2">
                                            <span class="inline-flex items-center gap-2 rounded-full bg-purple-500/10 px-3 py-1.5 text-xs font-semibold text-purple-700 ring-1 ring-purple-500/20 dark:text-purple-300">
                                                <span aria-hidden="true">
                                                    {content_type_icon(&detail.content_type)}
                                                </span>
                                                {format!("{:?}", detail.content_type)}
                                            </span>
                                            <span class="inline-flex items-center gap-2 rounded-full bg-black/5 px-3 py-1.5 text-xs font-medium text-foreground-600 ring-1 ring-black/5 dark:bg-white/10 dark:ring-white/10">
                                                <span aria-hidden="true">"⏱️"</span>
                                                {format!("{} min", detail.duration_minutes)}
                                            </span>
                                            {video_clock
                                                .map(|clock| {
                                                    view! {
                                                        <span class="inline-flex items-center gap-2 rounded-full bg-black/5 px-3 py-1.5 text-xs font-medium text-foreground-600 ring-1 ring-black/5 dark:bg-white/10 dark:ring-white/10">
                                                            <span aria-hidden="true">"▶️"</span>
                                                            {clock}
                                                        </span>
                                                    }
                                                })}
                                        </div>
                                    </header>

                                    <section class="relative isolate min-w-0 overflow-hidden rounded-3xl bg-linear-to-br from-purple-600/70 via-indigo-500/70 to-sky-500/70 p-px shadow-[0_24px_60px_-34px_rgba(79,70,229,0.8)]">
                                        <div class="relative min-w-0 space-y-5 rounded-3xl bg-surface-700 p-5 sm:p-6 lg:p-8">
                                            <div
                                                class="pointer-events-none absolute -right-20 -top-20 h-52 w-52 rounded-full bg-indigo-500/15 blur-3xl"
                                                aria-hidden="true"
                                            ></div>

                                            {video_url
                                                .map(|url| {
                                                    view! {
                                                        <div class="relative min-w-0 overflow-hidden rounded-2xl bg-black ring-1 ring-black/10 dark:ring-white/10">
                                                            <video
                                                                controls=true
                                                                src=url
                                                                class="block h-auto w-full"
                                                            ></video>
                                                        </div>
                                                    }
                                                })}

                                            {show_audio
                                                .then(|| audio_url.clone())
                                                .flatten()
                                                .map(|url| {
                                                    view! {
                                                        <div class="relative min-w-0 rounded-2xl bg-black/5 p-4 ring-1 ring-black/5 dark:bg-white/5 dark:ring-white/10">
                                                            <p class="mb-3 flex items-center gap-2 text-xs font-semibold uppercase tracking-wide text-foreground-600">
                                                                <span aria-hidden="true">"🎧"</span>
                                                                "Audio lesson"
                                                            </p>
                                                            <audio controls=true src=url class="w-full"></audio>
                                                        </div>
                                                    }
                                                })}

                                            {show_thumbnail
                                                .then(|| thumbnail_url.clone())
                                                .flatten()
                                                .map(|url| {
                                                    view! {
                                                        <div class="relative min-w-0 overflow-hidden rounded-2xl ring-1 ring-black/5 dark:ring-white/10">
                                                            <img
                                                                src=url
                                                                alt=detail.title.clone()
                                                                class="h-48 w-full object-cover sm:h-64 lg:h-80"
                                                            />
                                                        </div>
                                                    }
                                                })}

                                            <div class="relative min-w-0">
                                                <p class="min-w-0 whitespace-pre-line break-words text-base leading-relaxed text-foreground-900 sm:text-lg">
                                                    {detail.content.clone()}
                                                </p>
                                            </div>

                                            <div class="relative flex min-w-0 flex-wrap gap-3">
                                                {detail
                                                    .pdf_url
                                                    .clone()
                                                    .map(|url| {
                                                        view! {
                                                            <A
                                                                href=url
                                                                attr:target="_blank"
                                                                attr:rel="noopener noreferrer"
                                                                attr:class="group inline-flex min-h-[44px] min-w-0 items-center gap-2 rounded-xl bg-rose-500/10 px-4 text-sm font-semibold text-rose-700 transition duration-300 ease-out hover:-translate-y-0.5 hover:bg-rose-500/20 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-rose-400 dark:text-rose-300"
                                                            >
                                                                <span aria-hidden="true">"📕"</span>
                                                                <span class="min-w-0 break-words">
                                                                    "Open PDF (opens in new tab)"
                                                                </span>
                                                            </A>
                                                        }
                                                    })}

                                                {detail
                                                    .external_url
                                                    .clone()
                                                    .map(|url| {
                                                        view! {
                                                            <A
                                                                href=url
                                                                attr:target="_blank"
                                                                attr:rel="noopener noreferrer"
                                                                attr:class="group inline-flex min-h-[44px] min-w-0 items-center gap-2 rounded-xl bg-emerald-500/10 px-4 text-sm font-semibold text-emerald-700 transition duration-300 ease-out hover:-translate-y-0.5 hover:bg-emerald-500/20 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-emerald-400 dark:text-emerald-300"
                                                            >
                                                                <span aria-hidden="true">"🔗"</span>
                                                                <span class="min-w-0 break-words">
                                                                    "Open external resource"
                                                                </span>
                                                            </A>
                                                        }
                                                    })}
                                            </div>
                                        </div>
                                    </section>

                                    <section class="min-w-0 rounded-2xl bg-surface-700 p-5 shadow-md ring-1 ring-black/5 sm:p-6 dark:ring-white/10">
                                        {if is_enrolled {
                                            view! {
                                                <Show
                                                    when=move || completed.get()
                                                    fallback=move || {
                                                        view! {
                                                            <div class="min-w-0 space-y-3">
                                                                <div class="min-w-0 space-y-1">
                                                                    <p class="text-xs font-semibold uppercase tracking-wide text-foreground-600">
                                                                        "Finished this lesson?"
                                                                    </p>
                                                                    <p class="min-w-0 break-words text-sm text-foreground-600">
                                                                        "Marking it complete updates your course progress instantly."
                                                                    </p>
                                                                </div>

                                                                <button
                                                                    class="inline-flex min-h-[44px] w-full items-center justify-center gap-2 rounded-xl bg-indigo-400 px-5 font-semibold text-white shadow-lg transition duration-300 ease-out hover:-translate-y-0.5 hover:bg-indigo-500 hover:shadow-xl focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-indigo-300 disabled:translate-y-0 disabled:opacity-60 sm:w-auto"
                                                                    disabled=move || completing.get()
                                                                    on:click=on_complete
                                                                >
                                                                    <span aria-hidden="true">"✓"</span>
                                                                    "Mark as complete"
                                                                </button>

                                                                <Show when=move || completing.get()>
                                                                    <p class="text-sm font-medium text-foreground-600">"Saving…"</p>
                                                                </Show>

                                                                {error_banner}
                                                            </div>
                                                        }
                                                    }
                                                >
                                                    <div class="flex min-w-0 items-center gap-3 rounded-xl bg-emerald-500/10 p-4 ring-1 ring-emerald-500/20">
                                                        <span
                                                            class="grid h-10 w-10 shrink-0 place-items-center rounded-full bg-emerald-500/20 text-lg text-emerald-700 dark:text-emerald-300"
                                                            aria-hidden="true"
                                                        >
                                                            "✓"
                                                        </span>
                                                        <p class="min-w-0 break-words text-sm font-semibold text-emerald-700 dark:text-emerald-300">
                                                            "Lesson completed — progress updated"
                                                        </p>
                                                    </div>
                                                </Show>
                                            }
                                                .into_any()
                                        } else {
                                            view! {
                                                <div class="min-w-0 space-y-3">
                                                    <div class="flex min-w-0 items-start gap-3 rounded-xl bg-black/5 p-4 dark:bg-white/5">
                                                        <span class="shrink-0 text-lg" aria-hidden="true">"🔒"</span>
                                                        <div class="min-w-0 space-y-1">
                                                            <p class="min-w-0 break-words text-sm font-semibold text-foreground-900">
                                                                "You're not enrolled in this course"
                                                            </p>
                                                            <p class="min-w-0 break-words text-sm text-foreground-600">
                                                                "Enroll to track your progress and mark lessons as complete."
                                                            </p>
                                                        </div>
                                                    </div>

                                                    <A
                                                        href=enroll_href
                                                        attr:class="inline-flex min-h-[44px] w-full items-center justify-center gap-2 rounded-xl bg-indigo-400 px-5 font-semibold text-white shadow-lg transition duration-300 ease-out hover:-translate-y-0.5 hover:bg-indigo-500 hover:shadow-xl focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-indigo-300 sm:w-auto"
                                                    >
                                                        "Go to course to enroll"
                                                        <span aria-hidden="true">"→"</span>
                                                    </A>
                                                </div>
                                            }
                                                .into_any()
                                        }}
                                    </section>

                                    <nav class="flex min-w-0 flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
                                        {prev_lesson_id
                                            .map(|prev| {
                                                view! {
                                                    <A
                                                        href=format!("/learn/lessons/{}", prev)
                                                        attr:class="group inline-flex min-h-[44px] w-full items-center justify-center gap-2 rounded-xl bg-black/5 px-5 font-semibold text-foreground-900 transition duration-300 ease-out hover:-translate-x-0.5 hover:bg-indigo-400/15 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-indigo-400 dark:bg-white/10 sm:w-auto"
                                                    >
                                                        <span aria-hidden="true">"←"</span>
                                                        "Previous lesson"
                                                    </A>
                                                }
                                            })}

                                        {next_lesson_id
                                            .map(|next| {
                                                view! {
                                                    <A
                                                        href=format!("/learn/lessons/{}", next)
                                                        attr:class="group inline-flex min-h-[44px] w-full items-center justify-center gap-2 rounded-xl bg-indigo-400 px-5 font-semibold text-white shadow-lg transition duration-300 ease-out hover:translate-x-0.5 hover:bg-indigo-500 hover:shadow-xl focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-indigo-300 sm:ml-auto sm:w-auto"
                                                    >
                                                        "Next lesson"
                                                        <span aria-hidden="true">"→"</span>
                                                    </A>
                                                }
                                            })}
                                    </nav>
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
