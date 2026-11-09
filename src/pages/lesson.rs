use leptos::prelude::*;
use leptos::reactive::spawn_local;
use leptos_router::components::A;
use leptos_router::hooks::use_params_map;

use crate::components::mihrab::{MihrabDefs, MihrabStar};
use crate::models::education::LessonContentType;
use crate::models::gamification::{AchievementOnClient, UserStreakOnClient};
use crate::models::quiz::{QuestionType, QuizAnswer, QuizOnClient, QuizSubmission};
use crate::server_functions::education::complete_lesson;
use crate::server_functions::education::fetch_course_details;
use crate::server_functions::education::fetch_lesson_details;
use crate::server_functions::education::fetch_my_courses;
use crate::server_functions::education_gamification::fetch_achievements;
use crate::server_functions::education_gamification::fetch_streak;
use crate::server_functions::education_quiz::fetch_quiz_for_lesson;
use crate::server_functions::education_quiz::submit_quiz;

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

fn content_type_label(content_type: &LessonContentType) -> &'static str {
    match content_type {
        LessonContentType::Text => "Text",
        LessonContentType::Video => "Video",
        LessonContentType::Audio => "Audio",
        LessonContentType::Pdf => "PDF",
        LessonContentType::ExternalLink => "External link",
        LessonContentType::Mixed => "Mixed",
    }
}

fn format_clock(total_seconds: i32) -> String {
    let seconds = total_seconds.max(0);

    format!("{}:{:02}", seconds / 60, seconds % 60)
}

/// Fixed decorative waveform heights for the audio drill card.
const WAVE_BARS: [u8; 44] = [
    30, 60, 45, 85, 55, 95, 40, 70, 35, 80, 50, 65, 30, 75, 45, 60, 25, 55, 70, 40, 85, 50, 30,
    65, 90, 35, 55, 75, 45, 60, 25, 80, 40, 65, 85, 30, 55, 70, 45, 35, 60, 40, 50, 30,
];

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

    let streak = Resource::new(
        || (),
        |_| async move {
            fetch_streak()
                .await
                .ok()
                .and_then(|res| res.data)
        },
    );

    let achievements = Resource::new(
        || (),
        |_| async move {
            fetch_achievements()
                .await
                .ok()
                .and_then(|res| res.data)
                .unwrap_or_default()
        },
    );

    let course_modules = Resource::new(
        move || {
            lesson
                .get()
                .flatten()
                .map(|detail| detail.course_id.clone())
                .unwrap_or_default()
        },
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

    let quiz = Resource::new(
        move || lesson_id(),
        |lesson_id: String| async move {
            if lesson_id.is_empty() {
                return None;
            }

            fetch_quiz_for_lesson(lesson_id)
                .await
                .ok()
                .and_then(|res| res.data)
        },
    );

    let completing = RwSignal::new(false);
    let complete_error = RwSignal::new(None::<String>);
    let completed = RwSignal::new(false);
    let quiz_answers = RwSignal::new(std::collections::HashMap::<String, String>::new());
    let quiz_submitting = RwSignal::new(false);
    let quiz_result = RwSignal::new(None::<Result<(i32, i32), String>>);

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
                <div class="space-y-2 rounded-xl bg-rose-500/10 p-4 ring-1 ring-rose-500/20">
                    <p class="text-sm font-medium text-rose-700 break-words">{message}</p>

                    {needs_login
                        .then(|| {
                            view! {
                                <A
                                    href="/login"
                                    attr:class="inline-flex min-h-[44px] items-center gap-1 text-sm font-semibold text-sky-700 underline-offset-4 hover:underline"
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

    let on_quiz_submit = move |_| {
        if quiz_submitting.get_untracked() {
            return;
        }

        let answers_map = quiz_answers.get_untracked();
        let quiz_id = quiz
            .get_untracked()
            .flatten()
            .map(|quiz| quiz.id)
            .unwrap_or_default();

        if quiz_id.is_empty() {
            return;
        }

        quiz_submitting.set(true);
        quiz_result.set(None);

        let answers: Vec<QuizAnswer> = answers_map
            .into_iter()
            .map(|(question_id, answer)| QuizAnswer { question_id, answer })
            .collect();

        spawn_local(async move {
            match submit_quiz(QuizSubmission { quiz_id, answers }).await {
                Ok(response) => {
                    if let Some(error) = response.error {
                        quiz_result.set(Some(Err(error)));
                    } else if let Some(result) = response.data {
                        quiz_result.set(Some(Ok((result.correct_count, result.total_questions))));
                    } else {
                        quiz_result.set(Some(Err("No result from the server".to_string())));
                    }
                }
                Err(e) => quiz_result.set(Some(Err(e.to_string()))),
            }

            quiz_submitting.set(false);
        });
    };

    view! {
        <div class="py-4 sm:py-6">
            <MihrabDefs />

            <Suspense fallback=move || {
                view! {
                    <div class="mx-auto w-full max-w-[68rem] min-w-0 animate-pulse space-y-6">
                        <div class="h-10 w-56 rounded-xl bg-white"></div>
                        <div class="h-72 w-full rounded-3xl border border-mih-brass-soft bg-white shadow-sm"></div>
                        <div class="h-40 w-full rounded-2xl border border-mih-brass-soft bg-white"></div>
                        <p class="text-sm text-mih-muted">"Loading lesson…"</p>
                    </div>
                }
            }>
                {move || {
                    match lesson.get().flatten() {
                        None => {
                            view! {
                                <div class="mx-auto w-full max-w-[68rem] min-w-0 space-y-3 rounded-3xl border border-mih-brass-soft bg-white p-8 text-center shadow-sm">
                                    <p class="text-4xl" aria-hidden="true">"📭"</p>
                                    <h1 class="text-xl font-bold text-mih-ink sm:text-2xl">
                                        "Lesson not found."
                                    </h1>
                                    <p class="mx-auto max-w-sm text-sm text-mih-muted">
                                        "This lesson may have been removed or the link is out of date."
                                    </p>
                                    <A
                                        href="/learn"
                                        attr:class="inline-flex items-center justify-center gap-2 min-h-11 px-[1.15rem] rounded-[0.8rem] font-bold text-[0.9rem] border border-transparent bg-mih-indigo-strong text-white no-underline transition-[background-color,border-color,color,transform,box-shadow] duration-200 hover:bg-mih-indigo-deep hover:no-underline disabled:cursor-not-allowed disabled:opacity-[0.65]"
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
                            let is_enrolled = my_courses
                                .get()
                                .unwrap_or_default()
                                .iter()
                                .any(|row| row.course_id == detail.course_id);
                            let lesson_completed_on_server = detail.is_completed;
                            let is_complete = move || lesson_completed_on_server || completed.get();
                            let video_url = detail.video_url.clone();
                            let audio_url = detail.audio_url.clone();
                            let thumbnail_url = detail.thumbnail_url.clone();
                            let show_audio = video_url.is_none() && audio_url.is_some();
                            let show_thumbnail = video_url.is_none()
                                && audio_url.is_none()
                                && thumbnail_url.is_some();
                            let video_clock = detail.video_duration_seconds.map(format_clock);
                            let prev_lesson_id = detail.prev_lesson_id.clone();
                            let next_lesson_id = detail.next_lesson_id.clone();
                            let content_type_label = content_type_label(&detail.content_type);
                            let content_type_icon = content_type_icon(&detail.content_type);
                            let course_title = detail.course_title.clone();
                            let module_title = detail.module_title.clone();
                            let module_id = detail.module_id.clone();

                            view! {
                                <article class="mx-auto w-full max-w-[68rem] min-w-0 space-y-6">
                                    <nav class="flex flex-wrap items-center gap-2 text-sm text-mih-muted" aria-label="Breadcrumb">
                                        <A href="/learn" attr:class="hover:text-mih-indigo-strong">"Learning"</A>
                                        <span class="text-mih-brass">"/"</span>
                                        <A href=course_href.clone() attr:class="hover:text-mih-indigo-strong">
                                            {course_title.clone()}
                                        </A>
                                        <span class="text-mih-brass">"/"</span>
                                        <span>{module_title.clone()}</span>
                                    </nav>

                                    <header class="relative overflow-hidden rounded-[1.5rem] border border-mih-brass-soft bg-mih-card p-6 shadow-[0_0.75rem_2rem_rgba(31,20,58,0.08)]">
                                        <svg class="pointer-events-none absolute inset-0 h-full w-full opacity-[0.08] text-[#4c1d95]" aria-hidden="true">
                                            <rect width="100%" height="100%" fill="url(#mihrab-lattice)" />
                                        </svg>
                                        <div class="relative">
                                            <p class="text-[0.7rem] font-bold uppercase tracking-[0.18em] text-mih-brass">{module_title.clone()}</p>
                                            <h1 class="mt-1 break-words text-3xl font-bold leading-tight text-mih-ink text-balance">
                                                {detail.title.clone()}
                                            </h1>
                                            <div class="mt-4 flex flex-wrap gap-2">
                                                <span class="inline-flex items-center gap-[0.35rem] rounded-full border border-[#c7d2fe] bg-[#eef2ff] px-3 py-[0.3rem] text-xs font-semibold text-mih-indigo-deep">
                                                    <span aria-hidden="true">{content_type_icon}</span>
                                                    {content_type_label}
                                                </span>
                                                <span class="inline-flex items-center gap-[0.35rem] rounded-full border border-mih-brass-soft bg-mih-ivory px-3 py-[0.3rem] text-xs font-semibold text-mih-ink-2">{format!("{} min", detail.duration_minutes)}</span>
                                                {video_clock
                                                    .map(|clock| {
                                                        view! {
                                                            <span class="inline-flex items-center gap-[0.35rem] rounded-full border border-mih-brass-soft bg-mih-ivory px-3 py-[0.3rem] text-xs font-semibold text-mih-ink-2">{clock}</span>
                                                        }
                                                    })}
                                                {is_complete()
                                                    .then(|| {
                                                        view! { <span class="inline-flex items-center gap-[0.35rem] rounded-full border border-[#bbf7d0] bg-[#ecfdf5] px-3 py-[0.3rem] text-xs font-semibold text-[#166534]">"Completed"</span> }
                                                    })}
                                            </div>
                                        </div>
                                    </header>

                                    <div class="grid min-w-0 gap-6 lg:grid-cols-[minmax(0,1fr)_19rem]">
                                        <div class="min-w-0 space-y-6">
                                            {video_url
                                                .map(|url| {
                                                    let poster = thumbnail_url.clone();

                                                    view! {
                                                        <div class="relative p-[0.35rem] [clip-path:url(#mihrab-wide)] bg-[linear-gradient(170deg,rgba(184,135,59,0.55),rgba(184,135,59,0.12))]">
                                                            <div class="relative aspect-[16/10] w-full overflow-hidden grid place-items-center [clip-path:url(#mihrab-wide)] bg-[linear-gradient(170deg,#4338ca,#2e1065_60%,#1e1b4b)]">
                                                                <video
                                                                    controls=true
                                                                    src=url
                                                                    poster=poster.unwrap_or_default()
                                                                    class="h-full w-full object-cover"
                                                                ></video>
                                                            </div>
                                                        </div>
                                                    }
                                                })}

                                            {show_audio
                                                .then(|| audio_url.clone().unwrap_or_default())
                                                .map(|url| {
                                                    view! {
                                                        <section class="min-w-0 rounded-2xl border border-mih-brass-soft bg-mih-card p-[1.1rem_1.15rem] shadow-[0_0.75rem_2rem_rgba(31,20,58,0.08)]">
                                                            <p class="text-[0.7rem] font-bold uppercase tracking-[0.18em] text-mih-brass">"Audio lesson"</p>
                                                            <h3 class="mt-1 text-base font-bold text-mih-ink">"Listen, then recite along"</h3>
                                                            <div class="mt-3 flex items-center gap-3">
                                                                <span class="grid h-[2.9rem] w-[2.4rem] flex-none place-items-center bg-[#fdf6e7] [clip-path:url(#mihrab)]" aria-hidden="true">
                                                                    <svg width="16" height="16" viewBox="0 0 24 24" fill="#b8873b">
                                                                        <path d="M8 5v14l11-7z" />
                                                                    </svg>
                                                                </span>
                                                                <span class="flex h-8 min-w-0 flex-1 items-end gap-[3px]" aria-hidden="true">
                                                                    {WAVE_BARS
                                                                        .into_iter()
                                                                        .map(|height| {
                                                                            view! {
                                                                                <i class="min-w-[2px] flex-1 rounded-[2px] bg-[#e2d2b0]" style=format!("height: {height}%")></i>
                                                                            }
                                                                        })
                                                                        .collect_view()}
                                                                </span>
                                                            </div>
                                                            <audio controls=true src=url class="mt-3 w-full"></audio>
                                                        </section>
                                                    }
                                                })}

                                            {show_thumbnail
                                                .then(|| thumbnail_url.clone().unwrap_or_default())
                                                .map(|url| {
                                                    view! {
                                                        <div class="relative p-[0.35rem] [clip-path:url(#mihrab-wide)] bg-[linear-gradient(170deg,rgba(184,135,59,0.55),rgba(184,135,59,0.12))]">
                                                            <div class="relative aspect-[16/10] w-full overflow-hidden grid place-items-center [clip-path:url(#mihrab-wide)] bg-[linear-gradient(170deg,#4338ca,#2e1065_60%,#1e1b4b)]">
                                                                <img src=url alt=detail.title.clone() class="block h-full w-full object-cover" />
                                                            </div>
                                                        </div>
                                                    }
                                                })}

                                            {(!detail.content.is_empty())
                                                .then(|| {
                                                    view! {
                                                        <section class="min-w-0 rounded-2xl border border-mih-brass-soft bg-mih-card p-[1.1rem_1.15rem] shadow-[0_0.75rem_2rem_rgba(31,20,58,0.08)]">
                                                            <p class="font-amiri text-lg [direction:rtl] text-mih-brass [unicode-bidi:isolate]" >"ملاحظات الدرس"</p>
                                                            <h2 class="mt-0.5 text-base font-bold text-mih-ink">"Lesson notes"</h2>
                                                            <hr class="my-3 h-px border-0 bg-[linear-gradient(90deg,transparent,rgba(184,135,59,0.55),transparent)]" />
                                                            <p class="max-w-[68ch] whitespace-pre-line break-words text-base leading-relaxed text-mih-ink-2">
                                                                {detail.content.clone()}
                                                            </p>
                                                        </section>
                                                    }
                                                })}

                                            <div class="flex flex-wrap gap-3">
                                                {detail
                                                    .pdf_url
                                                    .clone()
                                                    .map(|url| {
                                                        view! {
                                                            <A
                                                                href=url
                                                                attr:target="_blank"
                                                                attr:rel="noopener noreferrer"
                                                                attr:class="inline-flex items-center justify-center gap-2 min-h-11 px-[1.15rem] rounded-[0.8rem] font-bold text-[0.9rem] border border-mih-brass-soft bg-mih-card text-mih-ink-2 no-underline transition-[background-color,border-color,color,transform,box-shadow] duration-200 hover:bg-mih-ivory hover:no-underline"
                                                            >
                                                                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
                                                                    <path d="M12 3v12m0 0 4-4m-4 4-4-4M4 19h16" />
                                                                </svg>
                                                                "Open PDF (opens in new tab)"
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
                                                                attr:class="inline-flex items-center justify-center gap-2 min-h-11 px-[1.15rem] rounded-[0.8rem] font-bold text-[0.9rem] border border-mih-brass-soft bg-mih-card text-mih-ink-2 no-underline transition-[background-color,border-color,color,transform,box-shadow] duration-200 hover:bg-mih-ivory hover:no-underline"
                                                            >
                                                                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
                                                                    <path d="M14 4h6v6M20 4 10 14M19 14v5a1 1 0 0 1-1 1H5a1 1 0 0 1-1-1V6a1 1 0 0 1 1-1h5" />
                                                                </svg>
                                                                "Open external resource"
                                                            </A>
                                                        }
                                                    })}
                                            </div>

                                            <section class="min-w-0 space-y-2">
                                                <div class="flex items-center gap-[0.6rem] text-mih-brass">
                                                    <h2 class="text-lg font-bold text-mih-ink">"Completion"</h2>
                                                    <hr class="h-px flex-1 border-0 bg-[linear-gradient(90deg,transparent,rgba(184,135,59,0.55),transparent)]" />
                                                </div>

                                                {if is_enrolled {
                                                    if is_complete() {
                                                        view! {
                                                            <div class="relative overflow-hidden rounded-2xl border border-mih-ring bg-mih-card px-[1.25rem] py-[1.1rem] shadow-[0_0.75rem_2rem_rgba(31,20,58,0.08)] [&::before]:absolute [&::before]:inset-x-0 [&::before]:top-0 [&::before]:h-[3px] [&::before]:bg-[linear-gradient(90deg,transparent,rgba(184,135,59,0.6),transparent)] [&::before]:content-['']">
                                                                <p class="text-[0.68rem] font-extrabold uppercase tracking-[0.16em] text-[#15803d]">"Completed"</p>
                                                                <div class="mt-2 flex items-center gap-3">
                                                                    <span class="grid h-10 w-10 flex-none place-items-center rounded-full bg-emerald-100" aria-hidden="true">
                                                                        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="#15803d" stroke-width="2.6" stroke-linecap="round" stroke-linejoin="round">
                                                                            <path d="M20 6 9 17l-5-5" />
                                                                        </svg>
                                                                    </span>
                                                                    <p class="text-sm font-semibold text-emerald-800">
                                                                        "Lesson completed — nice work."
                                                                    </p>
                                                                </div>
                                                                {next_lesson_id
                                                                    .clone()
                                                                    .map(|next| {
                                                                        view! {
                                                                            <A
                                                                                href=format!("/learn/lessons/{next}")
                                                                                attr:class="inline-flex items-center justify-center gap-2 min-h-11 px-[1.15rem] rounded-[0.8rem] font-bold text-[0.9rem] border border-transparent bg-mih-indigo-strong text-white no-underline transition-[background-color,border-color,color,transform,box-shadow] duration-200 hover:bg-mih-indigo-deep hover:no-underline disabled:cursor-not-allowed disabled:opacity-[0.65] mt-3 inline-flex"
                                                                            >
                                                                                "Next lesson"
                                                                                <span aria-hidden="true">"→"</span>
                                                                            </A>
                                                                        }
                                                                    })}
                                                            </div>
                                                        }
                                                        .into_any()
                                                    } else {
                                                        view! {
                                                            <div class="relative overflow-hidden rounded-2xl border border-mih-ring bg-mih-card px-[1.25rem] py-[1.1rem] shadow-[0_0.75rem_2rem_rgba(31,20,58,0.08)] [&::before]:absolute [&::before]:inset-x-0 [&::before]:top-0 [&::before]:h-[3px] [&::before]:bg-[linear-gradient(90deg,transparent,rgba(184,135,59,0.6),transparent)] [&::before]:content-['']">
                                                                <p class="text-[0.68rem] font-extrabold uppercase tracking-[0.16em] text-mih-indigo-strong">"Enrolled · not finished"</p>
                                                                <p class="mt-1 text-sm text-mih-muted">
                                                                    "Marking this complete moves your course progress forward."
                                                                </p>
                                                                <button
                                                                    class="inline-flex items-center justify-center gap-2 min-h-11 px-[1.15rem] rounded-[0.8rem] font-bold text-[0.9rem] border border-transparent bg-mih-indigo-strong text-white no-underline transition-[background-color,border-color,color,transform,box-shadow] duration-200 hover:bg-mih-indigo-deep hover:no-underline disabled:cursor-not-allowed disabled:opacity-[0.65] mt-3 w-full sm:w-auto"
                                                                    disabled=move || completing.get()
                                                                    on:click=on_complete
                                                                >
                                                                    <span aria-hidden="true">"✓"</span>
                                                                    "Mark as complete"
                                                                </button>
                                                                {move || {
                                                                    completing
                                                                        .get()
                                                                        .then(|| {
                                                                            view! { <p class="mt-2 text-sm font-medium text-mih-muted">"Saving…"</p> }
                                                                        })
                                                                }}
                                                                <div class="mt-3">{error_banner}</div>
                                                            </div>
                                                        }
                                                        .into_any()
                                                    }
                                                } else {
                                                    view! {
                                                        <div class="relative overflow-hidden rounded-2xl border border-mih-ring bg-mih-card px-[1.25rem] py-[1.1rem] shadow-[0_0.75rem_2rem_rgba(31,20,58,0.08)] [&::before]:absolute [&::before]:inset-x-0 [&::before]:top-0 [&::before]:h-[3px] [&::before]:bg-[linear-gradient(90deg,transparent,rgba(184,135,59,0.6),transparent)] [&::before]:content-['']">
                                                            <p class="text-[0.68rem] font-extrabold uppercase tracking-[0.16em] text-mih-brass">"Not enrolled · preview"</p>
                                                            <p class="mt-1 text-sm text-mih-muted">
                                                                "You're previewing this lesson. Enroll to save progress, keep your streak and earn badges."
                                                            </p>
                                                            <A
                                                                href=course_href.clone()
                                                                attr:class="inline-flex items-center justify-center gap-2 min-h-11 px-[1.15rem] rounded-[0.8rem] font-bold text-[0.9rem] border border-transparent bg-mih-indigo-strong text-white no-underline transition-[background-color,border-color,color,transform,box-shadow] duration-200 hover:bg-mih-indigo-deep hover:no-underline disabled:cursor-not-allowed disabled:opacity-[0.65] mt-3 w-full sm:w-auto"
                                                            >
                                                                "Go to course to enroll"
                                                                <span aria-hidden="true">"→"</span>
                                                            </A>
                                                        </div>
                                                    }
                                                    .into_any()
                                                }}
                                            </section>

                                            {move || {
                                                quiz
                                                    .get()
                                                    .flatten()
                                                    .map(|quiz_data: QuizOnClient| {
                                                        let question_count = quiz_data.questions.len();

                                                        view! {
                                                            <section class="min-w-0 space-y-4 rounded-2xl border border-mih-brass-soft bg-mih-card p-[1.1rem_1.15rem] shadow-[0_0.75rem_2rem_rgba(31,20,58,0.08)]">
                                                                <div class="flex flex-wrap items-baseline gap-3">
                                                                    <h2 class="text-lg font-bold text-mih-ink">"Check your understanding"</h2>
                                                                    <span class="text-xs text-mih-muted">
                                                                        {format!("{question_count} generated questions · pass at {}%", (quiz_data.passing_score * 100.0) as i32)}
                                                                    </span>
                                                                </div>
                                                                <hr class="h-px flex-1 border-0 bg-[linear-gradient(90deg,transparent,rgba(184,135,59,0.55),transparent)]" />
                                                                {quiz_data
                                                                    .questions
                                                                    .into_iter()
                                                                    .enumerate()
                                                                    .map(|(index, question)| {
                                                                        let question_id = question.id.clone();
                                                                        let options = match question.question_type {
                                                                            QuestionType::TrueFalse => {
                                                                                vec!["True".to_string(), "False".to_string()]
                                                                            }
                                                                            QuestionType::MultipleChoice => {
                                                                                question.options.clone().unwrap_or_default()
                                                                            }
                                                                        };
                                                                        let name = format!("quiz-q-{question_id}");
                                                                        let question_id_for_radio = question_id.clone();

                                                                        view! {
                                                                            <div class="rounded-[0.8rem] border border-mih-ring bg-mih-card p-[0.85rem_0.95rem]">
                                                                                <p class="font-bold leading-snug text-mih-ink text-sm">
                                                                                    {format!("{} · {}", index + 1, question.question_text)}
                                                                                </p>
                                                                                <div class="mt-2 grid gap-2">
                                                                                    {options
                                                                                        .into_iter()
                                                                                        .map(|option| {
                                                                                            let answer_value = option.clone();
                                                                                            let question_id = question_id_for_radio.clone();
                                                                                            let name = name.clone();

                                                                                            view! {
                                                                                                <label class="flex cursor-pointer items-center gap-2 rounded-[0.6rem] border border-mih-ring-soft px-[0.65rem] py-[0.5rem] text-[0.88rem] text-mih-ink-2 hover:bg-mih-ivory [&_input]:accent-mih-indigo-strong">
                                                                                                    <input
                                                                                                        type="radio"
                                                                                                        name=name
                                                                                                        value=answer_value.clone()
                                                                                                        on:change=move |_| {
                                                                                                            quiz_answers
                                                                                                                .update(|map| {
                                                                                                                    map.insert(question_id.clone(), answer_value.clone());
                                                                                                                });
                                                                                                        }
                                                                                                    />
                                                                                                    {option}
                                                                                                </label>
                                                                                            }
                                                                                        })
                                                                                        .collect_view()}
                                                                                </div>
                                                                            </div>
                                                                        }
                                                                    })
                                                                    .collect_view()}
                                                                <button
                                                                    class="inline-flex items-center justify-center gap-2 min-h-11 px-[1.15rem] rounded-[0.8rem] font-bold text-[0.9rem] border border-transparent bg-mih-indigo-strong text-white no-underline transition-[background-color,border-color,color,transform,box-shadow] duration-200 hover:bg-mih-indigo-deep hover:no-underline disabled:cursor-not-allowed disabled:opacity-[0.65]"
                                                                    disabled=move || quiz_submitting.get()
                                                                    on:click=on_quiz_submit
                                                                >
                                                                    {move || {
                                                                        if quiz_submitting.get() {
                                                                            "Submitting…"
                                                                        } else {
                                                                            "Submit answers"
                                                                        }
                                                                    }}
                                                                </button>
                                                                {move || {
                                                                    quiz_result
                                                                        .get()
                                                                        .map(|result| {
                                                                            match result {
                                                                                Ok((correct, total)) => {
                                                                                    view! {
                                                                                        <p class="rounded-xl border border-emerald-300 bg-emerald-50 p-3 text-sm font-semibold text-emerald-800">
                                                                                            {format!("Result: {correct} of {total} correct — check the notes and try again any time.")}
                                                                                        </p>
                                                                                    }
                                                                                    .into_any()
                                                                                }
                                                                                Err(message) => {
                                                                                    view! {
                                                                                        <p class="rounded-xl border border-rose-300 bg-rose-50 p-3 text-sm font-medium text-rose-800">
                                                                                            {message}
                                                                                        </p>
                                                                                    }
                                                                                    .into_any()
                                                                                }
                                                                            }
                                                                        })
                                                                }}
                                                            </section>
                                                        }
                                                    })
                                            }}

                                            <nav class="grid gap-3 sm:grid-cols-2">
                                                {prev_lesson_id
                                                    .map(|prev| {
                                                        view! {
                                                            <A
                                                                href=format!("/learn/lessons/{prev}")
                                                                attr:class="inline-flex items-center justify-center gap-2 min-h-11 px-[1.15rem] rounded-[0.8rem] font-bold text-[0.9rem] border border-mih-brass-soft bg-mih-card text-mih-ink-2 no-underline transition-[background-color,border-color,color,transform,box-shadow] duration-200 hover:bg-mih-ivory hover:no-underline min-h-[3rem]"
                                                            >
                                                                "← Previous lesson"
                                                            </A>
                                                        }
                                                    })}
                                                {next_lesson_id
                                                    .clone()
                                                    .map(|next| {
                                                        view! {
                                                            <A
                                                                href=format!("/learn/lessons/{next}")
                                                                attr:class="inline-flex items-center justify-center gap-2 min-h-11 px-[1.15rem] rounded-[0.8rem] font-bold text-[0.9rem] border border-transparent bg-mih-indigo-strong text-white no-underline transition-[background-color,border-color,color,transform,box-shadow] duration-200 hover:bg-mih-indigo-deep hover:no-underline disabled:cursor-not-allowed disabled:opacity-[0.65] min-h-[3rem] sm:col-start-2"
                                                            >
                                                                "Next lesson"
                                                                <span aria-hidden="true">"→"</span>
                                                            </A>
                                                        }
                                                    })}
                                            </nav>
                                        </div>

                                        <aside class="min-w-0 space-y-4">
                                            {move || {
                                                streak
                                                    .get()
                                                    .flatten()
                                                    .map(|streak_data: UserStreakOnClient| {
                                                        view! {
                                                            <section class="min-w-0 rounded-2xl border border-mih-brass-soft bg-mih-card p-[1.1rem_1.15rem] shadow-[0_0.75rem_2rem_rgba(31,20,58,0.08)]">
                                                                <p class="text-[0.7rem] font-bold uppercase tracking-[0.18em] text-mih-brass">"Your streak"</p>
                                                                <div class="mt-3 flex items-center gap-3">
                                                                    <span class="grid h-[2.9rem] w-[2.4rem] flex-none place-items-center bg-[#fdf6e7] [clip-path:url(#mihrab)]" aria-hidden="true">
                                                                        <svg width="16" height="16" viewBox="0 0 24 24" fill="#b8873b">
                                                                            <path d="M13 2s1 3-2 6-4 4.5-4 7a7 7 0 0 0 14 0c0-3-2-5-3-7-1.5 1.5-2 2-3 2 1-3 0-6-2-8z" />
                                                                        </svg>
                                                                    </span>
                                                                    <div class="min-w-0">
                                                                        <p class="text-lg font-bold leading-tight text-mih-ink">
                                                                            {format!("{}-day streak", streak_data.current_streak)}
                                                                        </p>
                                                                        <p class="text-xs text-mih-muted">
                                                                            {format!("Longest run: {} days", streak_data.longest_streak)}
                                                                        </p>
                                                                    </div>
                                                                </div>
                                                                {let earned = achievements
                                                                    .get()
                                                                    .unwrap_or_default()
                                                                    .into_iter()
                                                                    .filter(|achievement| achievement.earned_at.is_some())
                                                                    .take(3)
                                                                    .collect::<Vec<AchievementOnClient>>();
                                                                    if earned.is_empty() {
                                                                        view! {
                                                                            <p class="mt-3 text-xs text-mih-muted">
                                                                                "Earn badges by completing lessons."
                                                                            </p>
                                                                        }
                                                                        .into_any()
                                                                    } else {
                                                                        view! {
                                                                            <div class="mt-3 flex gap-2">
                                                                                {earned
                                                                                    .into_iter()
                                                                                    .map(|achievement| {
                                                                                        view! {
                                                                                            <span class="min-w-0 flex-1 truncate rounded-[0.75rem] border border-[rgba(184,135,59,0.6)] bg-[#fdf6e7] p-[0.5rem_0.35rem] text-center text-[0.66rem] font-bold text-[#8a6a06]">{achievement.name.clone()}</span>
                                                                                        }
                                                                                    })
                                                                                    .collect_view()}
                                                                            </div>
                                                                        }
                                                                        .into_any()
                                                                    }}
                                                                />
                                                            </section>
                                                        }
                                                    })
                                            }}

                                            {move || {
                                                course_modules
                                                    .get()
                                                    .flatten()
                                                    .and_then(|course_detail| {
                                                        course_detail
                                                            .modules
                                                            .into_iter()
                                                            .find(|module| module.id == module_id)
                                                    })
                                                    .map(|module| {
                                                        let module_total = module.lessons.len();
                                                        let module_done = module
                                                            .lessons
                                                            .iter()
                                                            .filter(|lesson| lesson.is_completed)
                                                            .count();
                                                        let module_progress = if module_total > 0 {
                                                            (module_done as f32 / module_total as f32 * 100.0).round() as i32
                                                        } else {
                                                            0
                                                        };
                                                        let module_title = module.title.clone();

                                                        view! {
                                                            <section class="min-w-0 rounded-2xl border border-mih-brass-soft bg-mih-card p-[1.1rem_1.15rem] shadow-[0_0.75rem_2rem_rgba(31,20,58,0.08)]">
                                                                <p class="text-[0.7rem] font-bold uppercase tracking-[0.18em] text-mih-brass">
                                                                    {format!("Module · {module_title}")}
                                                                </p>
                                                                <ol class="mt-3 grid grid-cols-1 gap-1.5 text-sm">
                                                                    {module
                                                                        .lessons
                                                                        .into_iter()
                                                                        .enumerate()
                                                                        .map(|(index, lesson)| {
                                                                            let lesson_id = lesson.id.clone();
                                                                            let href = format!("/learn/lessons/{lesson_id}");
                                                                            let lesson_is_done = lesson.is_completed;
                                                                            let lesson_is_current = !lesson_is_done && lesson.id == detail.id;

                                                                            view! {
                                                                                <li>
                                                                                    <A
                                                                                        href=href
                                                                                        attr:class=if lesson_is_current {
                                                                                            "flex min-w-0 items-center gap-2 rounded-lg bg-[#fdf6e7] px-2 py-1.5 no-underline"
                                                                                        } else {
                                                                                            "flex min-w-0 items-center gap-2 rounded-lg px-2 py-1.5 no-underline"
                                                                                        }
                                                                                    >
                                                                                        {if lesson_is_done {
                                                                                            view! {
                                                                                                <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="#15803d" stroke-width="2.6" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
                                                                                                    <path d="M20 6 9 17l-5-5" />
                                                                                                </svg>
                                                                                            }
                                                                                            .into_any()
                                                                                        } else if lesson_is_current {
                                                                                            view! { <MihrabStar class="w-3.5 h-3.5 flex-none text-mih-brass" /> }
                                                                                            .into_any()
                                                                                        } else {
                                                                                            view! { <span class="inline-block size-[0.62rem] rounded-full border-2 border-[#ddd3bd]" aria-hidden="true"></span> }
                                                                                            .into_any()
                                                                                        }}
                                                                                        <span class="min-w-0 flex-1 truncate text-mih-ink-2">
                                                                                            {format!("{}. {}", index + 1, lesson.title)}
                                                                                        </span>
                                                                                        <span class="ml-auto whitespace-nowrap text-xs text-mih-muted">
                                                                                            {format!("{} min", lesson.duration_minutes)}
                                                                                        </span>
                                                                                    </A>
                                                                                </li>
                                                                            }
                                                                        })
                                                                        .collect_view()}
                                                                </ol>
                                                                <div class="h-[0.4rem] overflow-hidden rounded-full border border-mih-brass-soft bg-[#f1efe6] mt-3">
                                                                    <i class="block h-full bg-[linear-gradient(90deg,#b8873b,#e0b25f)]" style=format!("width: {module_progress}%")></i>
                                                                </div>
                                                                <p class="mt-1 text-xs text-mih-muted">
                                                                    {format!("Module progress · {module_done} of {module_total} done")}
                                                                </p>
                                                                <A
                                                                    href=course_href.clone()
                                                                    attr:class="mt-2 inline-flex text-xs font-semibold text-sky-700 hover:underline"
                                                                >
                                                                    "Back to course →"
                                                                </A>
                                                            </section>
                                                        }
                                                    })
                                            }}
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
