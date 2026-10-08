use leptos::IntoView;
use leptos::prelude::*;

use crate::components::cards::EducationalResourceCard;
use crate::models::education::CourseOnClient;
use crate::models::education::EnrollmentProgress;
use crate::models::recommended_course_item::RecommendedCourseItem;
use crate::server_functions::education::fetch_my_courses;
use crate::server_functions::education::fetch_track_courses;
use crate::server_functions::education::fetch_tracks;

const PREVIEW_SIZE: usize = 5;

fn from_enrollment(progress: &EnrollmentProgress) -> RecommendedCourseItem {
    RecommendedCourseItem::new(
        progress.course_id.clone(),
        format!(
            "{}/{} lessons",
            progress.completed_lessons, progress.total_lessons
        ),
        "Enrolled".to_string(),
        progress.course_title.clone(),
        format!("{:.0}% complete", progress.progress_percent),
        "Continue Learning".to_string(),
        progress.thumbnail_url.clone(),
    )
}

fn from_course(course: &CourseOnClient) -> RecommendedCourseItem {
    RecommendedCourseItem::new(
        course.id.clone(),
        format!("{} lessons", course.lesson_count),
        format!("{:?}", course.level),
        course.title.clone(),
        course.educator_name.clone(),
        "Start Course".to_string(),
        course.thumbnail_url.clone(),
    )
}

async fn load_recommended() -> Vec<RecommendedCourseItem> {
    let enrolled: Vec<RecommendedCourseItem> = fetch_my_courses()
        .await
        .ok()
        .and_then(|res| res.data)
        .map(|progress| progress.iter().map(from_enrollment).collect())
        .unwrap_or_default();

    if !enrolled.is_empty() {
        return enrolled;
    }

    let first_track_id: Option<String> = fetch_tracks()
        .await
        .ok()
        .and_then(|res| res.data)
        .and_then(|tracks| tracks.into_iter().next().map(|track| track.id));

    match first_track_id {
        None => Vec::new(),
        Some(track_id) => fetch_track_courses(track_id)
            .await
            .ok()
            .and_then(|res| res.data)
            .map(|courses| courses.iter().map(from_course).collect())
            .unwrap_or_default(),
    }
}

#[component]
pub fn RecommendedSection(show_all: ReadSignal<bool>) -> impl IntoView {
    let courses = Resource::new(|| (), |_| async move { load_recommended().await });

    let visible = Memo::new(move |_| {
        let all = courses.get().unwrap_or_default();

        if show_all.get() {
            all
        } else {
            all.into_iter().take(PREVIEW_SIZE).collect()
        }
    });

    view! {
        <div class="flex gap-5 overflow-x-scroll pb-4">
            <For
                each=move || visible.get()
                key=|item| item.id.clone()
                let(item)
            >
                <EducationalResourceCard
                    lesson_count=item.lesson_count.clone()
                    level=item.level.clone()
                    resource_title=item.resource_title.clone()
                    resource_by=item.resource_by.clone()
                    action_label=item.action_label.clone()
                    image_url=item
                        .image_url
                        .clone()
                        .unwrap_or_else(|| {
                            "https://images.unsplash.com/photo-1542816417-0983c9c9ad53?auto=format&fit=crop&w=640&h=360&q=80"
                                .to_string()
                        })
                />
            </For>
            <Show when=move || visible.get().is_empty()>
                <p class="text-sm text-foreground-600">"No recommended courses yet."</p>
            </Show>
        </div>
    }
}
