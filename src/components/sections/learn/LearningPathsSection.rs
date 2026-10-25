use leptos::prelude::*;

use crate::components::cards::LearningPathCard;
use crate::models::education::EnrollmentProgress;
use crate::models::roadmap::RoadmapOnClient;
use crate::server_functions::education_roadmap::fetch_roadmap_detail;

const ICONS: [&str; 4] = ["🧭", "🗺️", "🚀", "📚"];

const TITLE_CLASSES: [&str; 4] = [
    "text-purple-900 dark:text-purple-100",
    "text-indigo-900 dark:text-indigo-100",
    "text-sky-900 dark:text-sky-100",
    "text-emerald-900 dark:text-emerald-100",
];

const DESCRIPTION_CLASSES: [&str; 4] = [
    "text-purple-700 dark:text-purple-300",
    "text-indigo-700 dark:text-indigo-300",
    "text-sky-700 dark:text-sky-300",
    "text-emerald-700 dark:text-emerald-300",
];

const ICON_CLASSES: [&str; 4] = [
    "bg-purple-500/15 text-purple-600 dark:text-purple-300",
    "bg-indigo-500/15 text-indigo-600 dark:text-indigo-300",
    "bg-sky-500/15 text-sky-600 dark:text-sky-300",
    "bg-emerald-500/15 text-emerald-600 dark:text-emerald-300",
];

fn average_progress(course_ids: &[String], enrolled: &[EnrollmentProgress]) -> Option<f32> {
    let matched = enrolled
        .iter()
        .filter(|row| course_ids.iter().any(|id| id == &row.course_id))
        .collect::<Vec<_>>();

    if matched.is_empty() {
        return None;
    }

    let total = matched
        .iter()
        .map(|row| row.progress_percent)
        .sum::<f32>();

    Some((total / matched.len() as f32).clamp(0.0, 100.0))
}

#[component]
pub fn LearningPathsSection(
    roadmaps: Vec<RoadmapOnClient>,
    enrolled: Vec<EnrollmentProgress>,
) -> impl IntoView {
    let is_empty = roadmaps.is_empty();

    let cards = roadmaps
        .into_iter()
        .enumerate()
        .map(|(index, roadmap)| {
            let enrolled_rows = enrolled.clone();
            let resource_id = roadmap.id.clone();
            let detail = Resource::new(
                move || resource_id.clone(),
                |roadmap_id: String| async move {
                    fetch_roadmap_detail(roadmap_id)
                        .await
                        .ok()
                        .and_then(|res| res.data)
                },
            );

            let style_index = index % ICONS.len();

            view! {
                <Suspense fallback=move || view! {
                    <p class="text-sm text-foreground-600">"Loading learning paths…"</p>
                }>
                    {move || {
                        let detail = detail.get().flatten();
                        let course_ids = detail
                            .as_ref()
                            .map(|detail| {
                                detail
                                    .courses
                                    .iter()
                                    .map(|course| course.course_id.clone())
                                    .collect::<Vec<_>>()
                            })
                            .unwrap_or_default();
                        let course_count = detail
                            .as_ref()
                            .map(|detail| detail.courses.len())
                            .unwrap_or(0);
                        let progress = average_progress(&course_ids, &enrolled_rows);

                        view! {
                            <LearningPathCard
                                title=roadmap.title.clone()
                                description=roadmap.description.clone()
                                title_class=TITLE_CLASSES[style_index].to_string()
                                description_class=DESCRIPTION_CLASSES[style_index].to_string()
                                icon=ICONS[style_index].to_string()
                                icon_class=ICON_CLASSES[style_index].to_string()
                                course_count=format!("{} courses", course_count)
                                duration=format!("{} weeks", roadmap.estimated_weeks)
                                progress_label=match progress {
                                    Some(percent) => format!("{percent:.0}%"),
                                    None => "Not Started".to_string(),
                                }
                                progress_percent=progress
                                    .unwrap_or(0.0)
                                    .clamp(0.0, 100.0)
                                    .round() as u8
                                cta_label=if progress.is_some() {
                                    "Continue Path".to_string()
                                } else {
                                    "Begin Path".to_string()
                                }
                                cta_href=format!("/learn/roadmaps/{}", roadmap.id)
                            />
                        }
                    }}
                </Suspense>
            }
        })
        .collect_view();

    view! {
        <div class="min-w-0 space-y-5">
            <div class="grid min-w-0 grid-cols-1 gap-5 sm:grid-cols-2 lg:grid-cols-3">
                {cards}
            </div>

            <Show when=move || is_empty>
                <p class="text-sm text-foreground-600">"No learning paths yet."</p>
            </Show>
        </div>
    }
}
