use std::cmp::Reverse;

use leptos::IntoView;
use leptos::prelude::*;

use crate::components::cards::ContinueLearningCard;
use crate::models::education::EnrollmentProgress;
use crate::server_functions::education::fetch_course_details;

fn select_course(courses: &[EnrollmentProgress]) -> Option<EnrollmentProgress> {
    let mut sorted = courses.to_vec();
    sorted.sort_by_key(|course| Reverse(course.last_accessed_at));

    sorted
        .iter()
        .find(|course| course.progress_percent > 0.0 && course.progress_percent < 100.0)
        .or_else(|| {
            sorted
                .iter()
                .find(|course| course.progress_percent < 100.0)
        })
        .cloned()
}

fn lesson_progress(progress: &EnrollmentProgress) -> String {
    if progress.total_lessons <= 0 {
        return "No lessons yet".to_string();
    }

    let current = (progress.completed_lessons + 1).min(progress.total_lessons);

    format!("Lesson {current} of {}", progress.total_lessons)
}

#[component]
pub fn ContinueLearningSection(courses: Memo<Vec<EnrollmentProgress>>) -> impl IntoView {
    let selected = Memo::new(move |_| select_course(&courses.get()));

    let details = Resource::new(
        move || selected.get().map(|course| course.course_id.clone()),
        |course_id: Option<String>| async move {
            match course_id {
                Some(course_id) => fetch_course_details(course_id)
                    .await
                    .ok()
                    .and_then(|res| res.data),
                None => None,
            }
        },
    );

    view! {
        <section>
            <Show
                when=move || selected.get().is_some()
                fallback=|| {
                    view! {
                        <p class="text-sm text-foreground-600">
                            "Enroll in a course to keep learning here."
                        </p>
                    }
                }
            >
                {move || {
                    let progress = selected.get();
                    let detail = details.get().flatten();
                    match (progress, detail) {
                        (Some(progress), Some(detail)) => {
                            let remaining =
                                (progress.total_lessons - progress.completed_lessons).max(0);
                            view! {
                                <ContinueLearningCard
                                    status="In Progress".to_string()
                                    title=detail.title.clone()
                                    description=detail.short_description.clone()
                                    time_remaining=format!("{remaining} lessons left")
                                    lesson_progress=lesson_progress(&progress)
                                    progress_percent=progress
                                        .progress_percent
                                        .clamp(0.0, 100.0)
                                        .round() as u8
                                    cta_label="Continue Learning".to_string()
                                    img_link=detail
                                        .thumbnail_url
                                        .clone()
                                        .unwrap_or_else(|| {
                                            "https://images.unsplash.com/photo-1501504905252-473c47e087f8?w=128&h=128&fit=crop"
                                                .to_string()
                                        })
                                />
                            }
                                .into_any()
                        }
                        _ => {
                            view! {
                                <p class="text-sm text-foreground-600">"Loading your course…"</p>
                            }
                                .into_any()
                        }
                    }
                }}
            </Show>
        </section>
    }
}
