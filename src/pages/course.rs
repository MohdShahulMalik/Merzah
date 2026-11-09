use leptos::prelude::*;
use leptos::reactive::spawn_local;
use leptos_router::components::A;
use leptos_router::hooks::use_params_map;

use crate::components::mihrab::{MihrabDefs, MihrabStar};
use crate::models::education::LessonContentType;
use crate::server_functions::education::enroll_course;
use crate::server_functions::education::fetch_course_details;
use crate::server_functions::education::fetch_course_progress;
use crate::server_functions::education::fetch_my_courses;
use crate::server_functions::education::fetch_track_courses;
use crate::server_functions::education::fetch_tracks;

const NOT_LOGGED_IN: &str = "You are not logged in";

fn format_duration(total_minutes: i32) -> String {
    let minutes = total_minutes.max(0);
    let hours = minutes / 60;
    let remainder = minutes % 60;

    if hours > 0 {
        format!("{hours}h {remainder}m")
    } else if remainder > 0 {
        format!("{remainder}m")
    } else {
        "—".to_string()
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

fn arabic_numeral(number: usize) -> String {
    const NUMERALS: [char; 9] = ['١', '٢', '٣', '٤', '٥', '٦', '٧', '٨', '٩'];

    if (1..=9).contains(&number) {
        NUMERALS[number - 1].to_string()
    } else {
        number.to_string()
    }
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

    let tracks = Resource::new(
        || (),
        |_| async move {
            fetch_tracks()
                .await
                .ok()
                .and_then(|res| res.data)
                .unwrap_or_default()
        },
    );

    let catalog = Resource::new(
        move || {
            tracks
                .get()
                .unwrap_or_default()
                .into_iter()
                .map(|track| track.id.clone())
                .collect::<Vec<String>>()
        },
        |track_ids: Vec<String>| async move {
            let mut entries = Vec::new();

            for track_id in track_ids {
                if let Some(courses) = fetch_track_courses(track_id).await.ok().and_then(|res| res.data) {
                    entries.extend(courses);
                }
            }

            entries
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
                <div class="space-y-2 rounded-xl bg-rose-500/10 p-4 ring-1 ring-rose-500/20">
                    <p class="text-sm font-medium text-rose-700 break-words">{message}</p>

                    {needs_login
                        .then(|| {
                            view! {
                                <A
                                    href="/login"
                                    attr:class="inline-flex min-h-[44px] items-center gap-1 text-sm font-semibold text-sky-700 underline-offset-4 hover:underline"
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
        <div class="py-4 sm:py-6">
            <MihrabDefs />

            <Suspense fallback=move || {
                view! {
                    <div class="mx-auto w-full max-w-[68rem] min-w-0 animate-pulse space-y-6">
                        <div class="h-10 w-56 rounded-xl bg-white"></div>
                        <div class="h-64 w-full rounded-3xl border border-mih-brass-soft bg-white shadow-sm sm:h-80"></div>
                        <div class="grid gap-6 lg:grid-cols-[minmax(0,1fr)_21rem]">
                            <div class="space-y-4">
                                <div class="h-28 w-full rounded-2xl border border-mih-brass-soft bg-white"></div>
                                <div class="h-40 w-full rounded-2xl border border-mih-brass-soft bg-white"></div>
                            </div>
                            <div class="h-56 w-full rounded-2xl border border-mih-brass-soft bg-white"></div>
                        </div>
                        <p class="text-sm text-mih-muted">"Loading course…"</p>
                    </div>
                }
            }>
                {move || {
                    match course.get().flatten() {
                        None => {
                            view! {
                                <div class="mx-auto w-full max-w-[68rem] min-w-0 space-y-3 rounded-3xl border border-mih-brass-soft bg-white p-8 text-center shadow-sm">
                                    <p class="text-4xl" aria-hidden="true">"🧐"</p>
                                    <h1 class="text-xl font-bold text-mih-ink sm:text-2xl">
                                        "Course not found."
                                    </h1>
                                    <p class="mx-auto max-w-sm text-sm text-mih-muted">
                                        "This course may have been unpublished or the link is out of date."
                                    </p>
                                    <A
                                        href="/learn"
                                        attr:class="inline-flex items-center justify-center gap-2 min-h-11 px-[1.15rem] rounded-[0.8rem] font-bold text-[0.9rem] border border-transparent bg-mih-indigo-strong text-white no-underline transition-[background-color,border-color,color,transform,box-shadow] duration-200 hover:bg-mih-indigo-deep hover:no-underline disabled:cursor-not-allowed disabled:opacity-[0.65]"
                                    >
                                        "Browse courses"
                                        <span aria-hidden="true">"→"</span>
                                    </A>
                                </div>
                            }
                            .into_any()
                        }
                        Some(detail) => {
                            let thumbnail = detail.thumbnail_url.clone();
                            let educator_name = detail.educator.display_name.clone();
                            let educator_initials = initials(&educator_name);
                            let modules = detail.modules.clone();
                            let module_count = modules.len();
                            let lesson_total = modules
                                .iter()
                                .map(|module| module.lessons.len())
                                .sum::<usize>();
                            let completed_lessons = modules
                                .iter()
                                .flat_map(|module| module.lessons.iter())
                                .filter(|lesson| lesson.is_completed)
                                .count();
                            let has_preview = modules
                                .iter()
                                .flat_map(|module| module.lessons.iter())
                                .any(|lesson| lesson.is_preview);
                            let first_incomplete = modules.iter().find_map(|module| {
                                module
                                    .lessons
                                    .iter()
                                    .find(|lesson| !lesson.is_completed)
                                    .map(|lesson| (module.title.clone(), lesson.id.clone(), lesson.title.clone()))
                            });
                            let first_incomplete_lesson_id = first_incomplete
                                .as_ref()
                                .map(|(_, lesson_id, _)| lesson_id.clone());
                            let next_lesson_title = first_incomplete
                                .as_ref()
                                .map(|(module_title, _, lesson_title)| {
                                    format!("{module_title} · {lesson_title}")
                                });
                            let first_incomplete_module_index = modules
                                .iter()
                                .position(|module| {
                                    module.lessons.iter().any(|lesson| !lesson.is_completed)
                                })
                                .unwrap_or(0);

                            let detail_id = detail.id.clone();
                            let rec_entries = catalog
                                .get()
                                .unwrap_or_default()
                                .into_iter()
                                .filter(|course| course.id != detail.id)
                                .take(3)
                                .collect::<Vec<_>>();
                            let course_title = detail.title.clone();
                            let short_description = detail.short_description.clone();
                            let duration_label = format_duration(detail.duration_minutes);
                            let lesson_label = lesson_total.to_string();
                            let module_label = module_count.to_string();
                            let enrolled_label = detail.enrollment_count.to_string();
                            let level_label = format!("{:?}", detail.level);
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
                            let enrollment_progress = my_courses
                                .get()
                                .unwrap_or_default()
                                .into_iter()
                                .find(|row| row.course_id == detail.id.clone());
                            let (completed_label, total_label) = enrollment_progress
                                .map(|row| (row.completed_lessons, row.total_lessons as i32))
                                .unwrap_or((completed_lessons as i32, lesson_total as i32));

                            view! {
                                <article class="mx-auto w-full max-w-[68rem] min-w-0 space-y-6">
                                    <A
                                        href="/learn"
                                        attr:class="inline-flex min-h-[44px] items-center gap-2 text-sm font-medium text-sky-700 hover:text-sky-800 dark:text-sky-300"
                                    >
                                        <span aria-hidden="true">"←"</span>
                                        "Back to Learn"
                                    </A>

                                    <header class="relative overflow-hidden rounded-[1.5rem] border border-mih-brass-soft bg-mih-card p-6 shadow-[0_0.75rem_2rem_rgba(31,20,58,0.08)]">
                                        <svg class="pointer-events-none absolute inset-0 h-full w-full opacity-[0.08] text-[#4c1d95]" aria-hidden="true">
                                            <rect width="100%" height="100%" fill="url(#mihrab-lattice)" />
                                        </svg>
                                        <div class="relative grid items-start gap-6 md:grid-cols-[13rem_minmax(0,1fr)]">
                                            <div class="w-44 sm:w-48 md:w-full">
                                                <div class="relative p-[0.35rem] [clip-path:url(#mihrab)] bg-[linear-gradient(170deg,rgba(184,135,59,0.55),rgba(184,135,59,0.12))]">
                                                    <div class="relative aspect-[3/4] w-full overflow-hidden grid place-items-center [clip-path:url(#mihrab)] bg-[radial-gradient(120%_140%_at_50%_-10%,#4c1d95_0%,#2e1065_45%,#1e1b4b_100%)]">
                                                        {match thumbnail {
                                                            Some(src) => view! {
                                                                <img src=src alt=course_title.clone() class="block h-full w-full object-cover" />
                                                            }
                                                                .into_any(),
                                                            None => view! {
                                                                <>
                                                                    <svg class="pointer-events-none absolute inset-0 h-full w-full text-mih-star opacity-20" aria-hidden="true">
                                                                        <rect width="100%" height="100%" fill="url(#mihrab-lattice)" />
                                                                    </svg>
                                                                    <span class="relative px-[0.85rem] text-center font-amiri text-[1.75rem] leading-[1.6] text-mih-crown [direction:rtl] [unicode-bidi:isolate]">
                                                                        "علم التجويد"
                                                                    </span>
                                                                </>
                                                            }
                                                                .into_any(),
                                                        }}
                                                    </div>
                                                </div>
                                            </div>

                                            <div class="min-w-0">
                                                <p class="text-[0.7rem] font-bold uppercase tracking-[0.18em] text-mih-brass">"Merzah · Learning"</p>
                                                <h1 class="mt-1 break-words text-3xl font-bold leading-tight text-mih-ink text-balance sm:text-4xl">
                                                    {detail.title.clone()}
                                                </h1>
                                                <p class="mt-[0.35rem] max-w-2xl break-words text-base leading-[1.55] text-mih-muted sm:text-lg">
                                                    {short_description}
                                                </p>

                                                <div class="mt-4 flex min-w-0 flex-wrap gap-2">
                                                    <span class="inline-flex items-center gap-[0.35rem] rounded-full border border-mih-brass-soft bg-mih-ivory px-3 py-[0.3rem] text-xs font-semibold text-mih-ink-2">{level_label}</span>
                                                    {has_preview
                                                        .then(|| {
                                                            view! { <span class="inline-flex items-center gap-[0.35rem] rounded-full border border-mih-brass-soft bg-mih-ivory px-3 py-[0.3rem] text-xs font-semibold text-mih-ink-2">"Free preview available"</span> }
                                                        })}
                                                </div>

                                                <div class="mt-5 grid grid-cols-4 max-sm:grid-cols-2 overflow-hidden rounded-[0.9rem] border border-mih-brass-soft bg-mih-ivory">
                                                    <div class="min-w-0 px-[0.85rem] py-[0.65rem]">
                                                        <b class="block text-[1.15rem] leading-[1.2]">{duration_label.clone()}</b>
                                                        <span class="mt-[0.15rem] block text-[0.62rem] font-bold uppercase tracking-[0.16em] text-mih-brass">"Duration"</span>
                                                    </div>
                                                    <div class="min-w-0 border-l border-mih-brass-soft px-[0.85rem] py-[0.65rem]">
                                                        <b class="block text-[1.15rem] leading-[1.2]">{lesson_label}</b>
                                                        <span class="mt-[0.15rem] block text-[0.62rem] font-bold uppercase tracking-[0.16em] text-mih-brass">"Lessons"</span>
                                                    </div>
                                                    <div class="min-w-0 border-l border-mih-brass-soft max-sm:border-l-0 max-sm:border-t max-sm:border-mih-brass-soft px-[0.85rem] py-[0.65rem]">
                                                        <b class="block text-[1.15rem] leading-[1.2]">{module_label}</b>
                                                        <span class="mt-[0.15rem] block text-[0.62rem] font-bold uppercase tracking-[0.16em] text-mih-brass">"Modules"</span>
                                                    </div>
                                                    <div class="min-w-0 border-l border-mih-brass-soft max-sm:border-t max-sm:border-mih-brass-soft px-[0.85rem] py-[0.65rem]">
                                                        <b class="block text-[1.15rem] leading-[1.2]">{enrolled_label}</b>
                                                        <span class="mt-[0.15rem] block text-[0.62rem] font-bold uppercase tracking-[0.16em] text-mih-brass">"Enrolled"</span>
                                                    </div>
                                                </div>
                                            </div>
                                        </div>
                                    </header>

                                    <section class="space-y-2">
                                        <div class="flex items-center gap-[0.6rem] text-mih-brass">
                                            <h2 class="text-lg font-bold text-mih-ink">"Enrollment"</h2>
                                            <hr class="h-px flex-1 border-0 bg-[linear-gradient(90deg,transparent,rgba(184,135,59,0.55),transparent)]" />
                                        </div>

                                        {match percent {
                                            Some(value) if value >= 100.0 => {
                                                view! {
                                                    <div class="relative overflow-hidden rounded-2xl border border-mih-ring bg-mih-card px-[1.25rem] py-[1.1rem] shadow-[0_0.75rem_2rem_rgba(31,20,58,0.08)] [&::before]:absolute [&::before]:inset-x-0 [&::before]:top-0 [&::before]:h-[3px] [&::before]:bg-[linear-gradient(90deg,transparent,rgba(184,135,59,0.6),transparent)] [&::before]:content-['']">
                                                        <p class="text-[0.68rem] font-extrabold uppercase tracking-[0.16em] text-[#15803d]">"Completed"</p>
                                                        <div class="mt-2 flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
                                                            <div>
                                                                <h3 class="text-base font-bold text-mih-ink">
                                                                    "100% — course completed"
                                                                </h3>
                                                                <p class="mt-1 text-sm text-mih-muted">
                                                                    "Every lesson is done. Revisit any module below whenever you want to refresh."
                                                                </p>
                                                                <div class="h-[0.4rem] max-w-xs overflow-hidden rounded-full border border-mih-brass-soft bg-[#f1efe6] mt-3">
                                                                    <i class="block h-full bg-[linear-gradient(90deg,#b8873b,#e0b25f)]" style="width: 100%"></i>
                                                                </div>
                                                                <p class="mt-1 text-xs font-medium text-mih-muted">
                                                                    {format!("{completed_label} of {total_label} lessons done")}
                                                                </p>
                                                            </div>
                                                            <span class="inline-flex items-center justify-center gap-2 min-h-11 px-[1.15rem] rounded-[0.8rem] font-bold text-[0.9rem] border border-[rgba(184,135,59,0.5)] bg-mih-ivory text-[#8a6a06] no-underline transition-[background-color,border-color,color,transform,box-shadow] duration-200 flex-none">
                                                                <MihrabStar class="w-4 h-4"/>
                                                                "Badge earned"
                                                            </span>
                                                        </div>
                                                    </div>
                                                }
                                                .into_any()
                                            }
                                            Some(value) => {
                                                view! {
                                                    <div class="relative overflow-hidden rounded-2xl border border-mih-ring bg-mih-card px-[1.25rem] py-[1.1rem] shadow-[0_0.75rem_2rem_rgba(31,20,58,0.08)] [&::before]:absolute [&::before]:inset-x-0 [&::before]:top-0 [&::before]:h-[3px] [&::before]:bg-[linear-gradient(90deg,transparent,rgba(184,135,59,0.6),transparent)] [&::before]:content-['']">
                                                        <p class="text-[0.68rem] font-extrabold uppercase tracking-[0.16em] text-mih-indigo-strong">"Enrolled · in progress"</p>
                                                        <div class="mt-2 flex flex-col gap-3 lg:flex-row lg:items-center lg:justify-between">
                                                            <div class="min-w-0">
                                                                <h3 class="text-base font-bold text-mih-ink">
                                                                    {format!("{value:.0}% complete")}
                                                                </h3>
                                                                {next_lesson_title
                                                                    .map(|next| {
                                                                        view! {
                                                                            <p class="mt-1 break-words text-sm text-mih-muted">
                                                                                "Next up — " {next}
                                                                            </p>
                                                                        }
                                                                    })}
                                                                <div class="mt-3 h-[0.4rem] max-w-xs overflow-hidden rounded-full border border-mih-brass-soft bg-[#f1efe6]">
                                                                    <i class="block h-full bg-[linear-gradient(90deg,#6366f1,#4338ca)]" style=format!("width: {value:.0}%")></i>
                                                                </div>
                                                                <p class="mt-1 text-xs font-medium text-mih-muted">
                                                                    {format!("{completed_label} of {total_label} lessons done")}
                                                                </p>
                                                            </div>
                                                            {first_incomplete_lesson_id
                                                                .map(|lesson_id| {
                                                                    let href = format!("/learn/lessons/{lesson_id}");

                                                                    view! {
                                                                        <A
                                                                            href=href
                                                                            attr:class="inline-flex items-center justify-center gap-2 min-h-11 px-[1.15rem] rounded-[0.8rem] font-bold text-[0.9rem] border border-transparent bg-mih-indigo-strong text-white no-underline transition-[background-color,border-color,color,transform,box-shadow] duration-200 hover:bg-mih-indigo-deep hover:no-underline disabled:cursor-not-allowed disabled:opacity-[0.65] flex-none"
                                                                        >
                                                                            "Continue"
                                                                            <span aria-hidden="true">"→"</span>
                                                                        </A>
                                                                    }
                                                                })}
                                                        </div>
                                                    </div>
                                                }
                                                .into_any()
                                            }
                                            None => {
                                                view! {
                                                    <div class="relative overflow-hidden rounded-2xl border border-mih-ring bg-mih-card px-[1.25rem] py-[1.1rem] shadow-[0_0.75rem_2rem_rgba(31,20,58,0.08)] [&::before]:absolute [&::before]:inset-x-0 [&::before]:top-0 [&::before]:h-[3px] [&::before]:bg-[linear-gradient(90deg,transparent,rgba(184,135,59,0.6),transparent)] [&::before]:content-['']">
                                                        <p class="text-[0.68rem] font-extrabold uppercase tracking-[0.16em] text-mih-brass">"Ready when you are"</p>
                                                        <div class="mt-2 flex flex-col gap-3 lg:flex-row lg:items-center lg:justify-between">
                                                            <div class="min-w-0">
                                                                <h3 class="text-base font-bold text-mih-ink">
                                                                    "Enroll and begin"
                                                                </h3>
                                                                <p class="mt-1 break-words text-sm text-mih-muted">
                                                                    "Free to join. Progress, streak and badges are saved to your Merzah account."
                                                                </p>
                                                                <div class="h-[0.4rem] max-w-xs overflow-hidden rounded-full border border-mih-brass-soft bg-[#f1efe6] mt-3">
                                                                    <i class="block h-full bg-[linear-gradient(90deg,#b8873b,#e0b25f)]" style="width: 0%"></i>
                                                                </div>
                                                                <p class="mt-1 text-xs font-medium text-mih-muted">
                                                                    {format!("0% · 0 of {lesson_total} lessons")}
                                                                </p>
                                                            </div>
                                                            <button
                                                                class="inline-flex items-center justify-center gap-2 min-h-11 px-[1.15rem] rounded-[0.8rem] font-bold text-[0.9rem] border border-transparent bg-mih-indigo-strong text-white no-underline transition-[background-color,border-color,color,transform,box-shadow] duration-200 hover:bg-mih-indigo-deep hover:no-underline disabled:cursor-not-allowed disabled:opacity-[0.65] flex-none"
                                                                disabled=move || enrolling.get()
                                                                on:click=on_enroll
                                                            >
                                                                "Enroll and begin"
                                                                <span aria-hidden="true">"→"</span>
                                                            </button>
                                                        </div>
                                                        {move || {
                                                            enrolling
                                                                .get()
                                                                .then(|| {
                                                                    view! {
                                                                        <p class="mt-3 text-sm font-medium text-mih-muted">"Enrolling…"</p>
                                                                    }
                                                                })
                                                        }}
                                                        <div class="mt-3">{error_banner}</div>
                                                    </div>
                                                }
                                                .into_any()
                                            }
                                        }}
                                    </section>

                                    <div class="grid min-w-0 gap-6 lg:grid-cols-[minmax(0,1fr)_20rem]">
                                        <div class="min-w-0 space-y-6">
                                            <section class="min-w-0 rounded-2xl border border-mih-brass-soft bg-mih-card p-[1.1rem_1.15rem] shadow-[0_0.75rem_2rem_rgba(31,20,58,0.08)]">
                                                <p class="font-amiri text-xl [direction:rtl] text-mih-brass [unicode-bidi:isolate]" >"عن الدورة"</p>
                                                <h2 class="mt-0.5 text-lg font-bold text-mih-ink">"About this course"</h2>
                                                <hr class="my-3 h-px border-0 bg-[linear-gradient(90deg,transparent,rgba(184,135,59,0.55),transparent)]" />
                                                <p class="max-w-[68ch] whitespace-pre-line break-words leading-relaxed text-mih-ink-2">
                                                    {detail.description.clone()}
                                                </p>
                                            </section>

                                            <section class="min-w-0 space-y-4">
                                                <div class="flex items-center gap-[0.6rem] text-mih-brass">
                                                    <h2 class="text-lg font-bold text-mih-ink">"Curriculum"</h2>
                                                    <span class="text-xs font-medium text-mih-muted">
                                                        {format!("{module_count} modules · {lesson_total} lessons · {duration_label}")}
                                                    </span>
                                                    <hr class="h-px flex-1 border-0 bg-[linear-gradient(90deg,transparent,rgba(184,135,59,0.55),transparent)]" />
                                                </div>

                                                {(lesson_total == 0)
                                                    .then(|| {
                                                        view! {
                                                            <div class="rounded-2xl border border-dashed border-mih-brass-soft bg-white p-8 text-center">
                                                                <p class="text-3xl" aria-hidden="true">"🪴"</p>
                                                                <p class="mt-2 text-sm font-semibold text-mih-ink">
                                                                    "No modules published yet."
                                                                </p>
                                                                <p class="mt-1 text-sm text-mih-muted">
                                                                    "The educator is still shaping this curriculum — check back soon."
                                                                </p>
                                                            </div>
                                                        }
                                                    })}

                                                <div class="min-w-0 space-y-3">
                                                    {modules
                                                        .clone()
                                                        .into_iter()
                                                        .enumerate()
                                                        .map(|(module_index, module)| {
                                                            let lesson_total_module = module.lessons.len();
                                                            let module_minutes: i32 = module
                                                                .lessons
                                                                .iter()
                                                                .map(|lesson| lesson.duration_minutes)
                                                                .sum();
                                                            let open = module_index <= first_incomplete_module_index;

                                                            view! {
                                                                <details class="overflow-hidden rounded-2xl border border-mih-ring bg-mih-card shadow-[0_0.75rem_2rem_rgba(31,20,58,0.08)]" open=open>
                                                                    <summary class="flex cursor-pointer list-none items-center gap-[0.85rem] border-b border-transparent px-[1.1rem] py-[0.85rem] [&::-webkit-details-marker]:hidden [details[open]_&]:border-mih-brass/30">
                                                                        <span class="grid h-[2.6rem] w-[2.1rem] flex-none place-items-center bg-[linear-gradient(170deg,#4c1d95,#1e1b4b)] font-amiri text-[1rem] font-bold text-[#fde68a] [direction:rtl] [clip-path:url(#mihrab-niche)] [unicode-bidi:isolate]">{arabic_numeral(module_index + 1)}</span>
                                                                        <span class="min-w-0">
                                                                            <h4 class="text-base font-bold text-mih-ink">{module.title.clone()}</h4>
                                                                            <span class="text-xs text-mih-muted">{format!("{lesson_total_module} lessons")}</span>
                                                                        </span>
                                                                        <span class="ml-auto whitespace-nowrap text-sm font-medium text-mih-brass">
                                                                            {format!("{module_minutes} min")}
                                                                        </span>
                                                                    </summary>
                                                                    <div>
                                                                        {module
                                                                            .lessons
                                                                            .clone()
                                                                            .into_iter()
                                                                            .map(|lesson| {
                                                                                let is_preview = lesson.is_preview;
                                                                                let is_completed = lesson.is_completed;
                                                                                let is_current = !is_completed
                                                                                    && Some(&lesson.id) == first_incomplete
                                                                                        .as_ref()
                                                                                        .map(|(_, id, _)| id);
                                                                                let href = format!("/learn/lessons/{}", lesson.id);
                                                                                let lesson_row_class = if is_current {
                                                                                    "flex min-h-[3.4rem] items-center gap-[0.8rem] border-t border-mih-ring-soft px-[1.1rem] py-[0.72rem] text-inherit no-underline first:border-t-0 bg-[#fdf6e7] hover:bg-[#fbeed3] focus-visible:-outline-offset-2 focus-visible:outline-2 focus-visible:outline-mih-indigo transition-colors duration-200"
                                                                                } else {
                                                                                    "flex min-h-[3.4rem] items-center gap-[0.8rem] border-t border-mih-ring-soft px-[1.1rem] py-[0.72rem] text-inherit no-underline first:border-t-0 hover:bg-mih-ivory focus-visible:-outline-offset-2 focus-visible:outline-2 focus-visible:outline-mih-indigo transition-colors duration-200"
                                                                                };

                                                                                view! {
                                                                                    <a href=href class=lesson_row_class>
                                                                                        <span class="grid size-6 flex-none place-items-center">
                                                                                            {if is_completed {
                                                                                                view! {
                                                                                                    <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="#15803d" stroke-width="2.6" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
                                                                                                        <path d="M20 6 9 17l-5-5" />
                                                                                                    </svg>
                                                                                                }
                                                                                                .into_any()
                                                                                            } else if is_current {
                                                                                                view! { <MihrabStar class="w-5 h-5 text-mih-brass" /> }
                                                                                                    .into_any()
                                                                                            } else {
                                                                                                view! { <span class="inline-block size-[0.62rem] rounded-full border-2 border-[#ddd3bd]" aria-hidden="true"></span> }
                                                                                                    .into_any()
                                                                                            }}
                                                                                        </span>
                                                                                        <span class="min-w-0 flex-1">
                                                                                            <span class="block break-words text-sm font-semibold text-mih-ink">
                                                                                                {lesson.title.clone()}
                                                                                            </span>
                                                                                            <span class="mt-1 flex flex-wrap gap-1.5">
                                                                                                <span class="inline-flex items-center gap-1 rounded-full border border-mih-brass-soft bg-mih-ivory px-[0.55rem] py-[0.1rem] text-[0.68rem] font-bold text-mih-muted">{content_type_label(&lesson.content_type)}</span>
                                                                                                {is_preview
                                                                                                    .then(|| {
                                                                                                        view! { <span class="inline-flex items-center gap-1 rounded-full border border-[rgba(184,135,59,0.55)] bg-[#fdf6e7] px-[0.55rem] py-[0.1rem] text-[0.68rem] font-bold text-[#8a6a06]">"Free preview"</span> }
                                                                                                    })}
                                                                                                {is_completed
                                                                                                    .then(|| {
                                                                                                        view! { <span class="inline-flex items-center gap-1 rounded-full border border-[#bbf7d0] bg-[#ecfdf5] px-[0.55rem] py-[0.1rem] text-[0.68rem] font-bold text-[#166534]">"Done"</span> }
                                                                                                    })}
                                                                                                {is_current
                                                                                                    .then(|| {
                                                                                                        view! { <span class="inline-flex items-center gap-1 rounded-full border border-[#c7d2fe] bg-[#eef2ff] px-[0.55rem] py-[0.1rem] text-[0.68rem] font-bold text-mih-indigo-strong">"Current · playing"</span> }
                                                                                                    })}
                                                                                            </span>
                                                                                        </span>
                                                                                        <span class="whitespace-nowrap text-xs font-medium text-mih-muted">
                                                                                            {format!("{} min", lesson.duration_minutes)}
                                                                                        </span>
                                                                                    </a>
                                                                                }
                                                                            })
                                                                            .collect_view()}
                                                                    </div>
                                                                </details>
                                                            }
                                                        })
                                                        .collect_view()}
                                                </div>
                                            </section>
                                        </div>

                                        <aside class="min-w-0 space-y-4 lg:sticky lg:top-6">
                                            <section class="min-w-0 rounded-2xl border border-mih-brass-soft bg-mih-card p-[1.1rem_1.15rem] shadow-[0_0.75rem_2rem_rgba(31,20,58,0.08)]">
                                                <p class="text-[0.7rem] font-bold uppercase tracking-[0.18em] text-mih-brass">"Educator"</p>
                                                <div class="mt-3 flex items-center gap-3">
                                                    <span class="grid h-[3.2rem] w-[2.6rem] flex-none place-items-center bg-[linear-gradient(170deg,#4c1d95,#1e1b4b)] font-amiri text-[1.1rem] font-bold text-[#fde68a] [direction:rtl] [clip-path:url(#mihrab)] [unicode-bidi:isolate]">
                                                        {educator_initials}
                                                    </span>
                                                    <div class="min-w-0">
                                                        <h3 class="break-words font-bold leading-snug text-mih-ink">
                                                            {educator_name}
                                                        </h3>
                                                        <p class="text-xs text-mih-muted">"Curriculum educator"</p>
                                                    </div>
                                                </div>
                                            </section>

                                            <section class="min-w-0 rounded-2xl border border-mih-brass-soft bg-mih-card p-[1.1rem_1.15rem] shadow-[0_0.75rem_2rem_rgba(31,20,58,0.08)]">
                                                <p class="text-[0.7rem] font-bold uppercase tracking-[0.18em] text-mih-brass">"At a glance"</p>
                                                <ul class="mt-3 grid gap-2 text-sm">
                                                    <li class="flex items-baseline justify-between gap-3">
                                                        <span class="text-mih-muted">"Level"</span>
                                                        <b class="text-mih-ink-2">{format!("{:?}", detail.level)}</b>
                                                    </li>
                                                    <li class="flex items-baseline justify-between gap-3">
                                                        <span class="text-mih-muted">"Duration"</span>
                                                        <b class="text-mih-ink-2">{format_duration(detail.duration_minutes)}</b>
                                                    </li>
                                                    <li class="flex items-baseline justify-between gap-3">
                                                        <span class="text-mih-muted">"Lessons"</span>
                                                        <b class="text-mih-ink-2">{lesson_total}</b>
                                                    </li>
                                                    <li class="flex items-baseline justify-between gap-3">
                                                        <span class="text-mih-muted">"Modules"</span>
                                                        <b class="text-mih-ink-2">{module_count}</b>
                                                    </li>
                                                    <li class="flex items-baseline justify-between gap-3">
                                                        <span class="text-mih-muted">"Enrolled"</span>
                                                        <b class="text-mih-ink-2">{detail.enrollment_count}</b>
                                                    </li>
                                                </ul>
                                            </section>
                                        </aside>
                                    </div>

                                    <section class="min-w-0 space-y-4">
                                        <div class="flex items-center gap-[0.6rem] text-mih-brass">
                                            <hr class="h-px flex-1 border-0 bg-[linear-gradient(90deg,transparent,rgba(184,135,59,0.55),transparent)]" />
                                            <h2 class="text-lg font-bold text-mih-ink">"Recommended next"</h2>
                                            <hr class="h-px flex-1 border-0 bg-[linear-gradient(90deg,transparent,rgba(184,135,59,0.55),transparent)]" />
                                        </div>
                                        {if rec_entries.is_empty() {
                                            view! {
                                                <div class="min-w-0 rounded-2xl border border-dashed border-mih-brass-soft bg-white p-6 text-center">
                                                    <p class="text-sm text-mih-muted">
                                                        "More courses are being curated — check back soon."
                                                    </p>
                                                </div>
                                            }
                                            .into_any()
                                        } else {
                                            view! {
                                                <div class="grid min-w-0 gap-4 sm:grid-cols-2 lg:grid-cols-3">
                                                    {rec_entries
                                                        .into_iter()
                                                        .map(|entry| {
                                                            let href = format!("/learn/courses/{}", entry.id);
                                                            let level_label = format!("{:?}", entry.level);

                                                            view! {
                                                                <article class="min-w-0 rounded-2xl border border-mih-brass-soft bg-mih-card p-[1.1rem_1.15rem] shadow-[0_0.75rem_2rem_rgba(31,20,58,0.08)]">
                                                                    <div class="relative grid h-28 place-items-center overflow-hidden rounded-xl bg-linear-to-br from-violet-800 to-indigo-950">
                                                                        <svg class="absolute inset-0 h-full w-full opacity-20" aria-hidden="true">
                                                                            <rect width="100%" height="100%" fill="url(#mihrab-lattice)" style="color:#fde68a" />
                                                                        </svg>
                                                                        <span class="relative text-3xl text-mih-crown">"✦"</span>
                                                                    </div>
                                                                    <div class="mt-3 flex flex-wrap gap-1.5">
                                                                        <span class="inline-flex items-center gap-1 rounded-full border border-mih-brass-soft bg-mih-ivory px-[0.55rem] py-[0.1rem] text-[0.68rem] font-bold text-mih-muted">{level_label}</span>
                                                                        <span class="inline-flex items-center gap-1 rounded-full border border-mih-brass-soft bg-mih-ivory px-[0.55rem] py-[0.1rem] text-[0.68rem] font-bold text-mih-muted">{format!("{} lessons", entry.lesson_count)}</span>
                                                                    </div>
                                                                    <h3 class="mt-2 break-words font-bold leading-snug text-mih-ink">
                                                                        {entry.title.clone()}
                                                                    </h3>
                                                                    <p class="mt-1 break-words text-xs text-mih-muted">
                                                                        {entry.educator_name.clone()}
                                                                    </p>
                                                                    <A
                                                                        href=href
                                                                        attr:class="inline-flex items-center justify-center gap-2 min-h-11 px-[1.15rem] rounded-[0.8rem] font-bold text-[0.9rem] border border-transparent bg-mih-indigo-strong text-white no-underline transition-[background-color,border-color,color,transform,box-shadow] duration-200 hover:bg-mih-indigo-deep hover:no-underline disabled:cursor-not-allowed disabled:opacity-[0.65] mt-3 w-full"
                                                                    >
                                                                        "View course"
                                                                        <span aria-hidden="true">"→"</span>
                                                                    </A>
                                                                </article>
                                                            }
                                                        })
                                                        .collect_view()}
                                                </div>
                                            }
                                            .into_any()
                                        }}
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
