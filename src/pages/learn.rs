use leptos::prelude::*;
use leptos_router::components::A;

use crate::components::filters::{FilterGroup, FilterOption, Filters};
use crate::components::sections::ContinueLearningSection;
use crate::components::sections::FeaturedCoursesSection;
use crate::components::sections::LearningPathsSection;
use crate::components::sections::RecommendedCoursesSection;
use crate::models::catalog_entry::CatalogEntry;
use crate::server_functions::education::fetch_my_courses;
use crate::server_functions::education::fetch_track_courses;
use crate::server_functions::education::fetch_tracks;

#[component]
pub fn Learn() -> impl IntoView {
    let enrolled = Resource::new(|| (), |_| async move {
        fetch_my_courses()
            .await
            .ok()
            .and_then(|res| res.data)
            .unwrap_or_default()
    });

    let tracks = Resource::new(|| (), |_| async move {
        fetch_tracks()
            .await
            .ok()
            .and_then(|res| res.data)
            .unwrap_or_default()
    });

    let catalog = Resource::new(
        move || {
            tracks
                .get()
                .unwrap_or_default()
                .into_iter()
                .map(|track| (track.id.clone(), track.name.clone()))
                .collect::<Vec<_>>()
        },
        |pairs: Vec<(String, String)>| async move {
            let mut entries = Vec::new();

            for (track_id, track_name) in pairs {
                let courses = fetch_track_courses(track_id.clone())
                    .await
                    .ok()
                    .and_then(|res| res.data)
                    .unwrap_or_default();

                for course in courses {
                    entries.push(CatalogEntry::new(
                        track_id.clone(),
                        track_name.clone(),
                        course,
                    ));
                }
            }

            entries
        },
    );

    let filter_groups = vec![
        FilterGroup::new(
            "Category:",
            vec![
                FilterOption::new("All", true),
                FilterOption::new("Faith & Worship", false),
                FilterOption::new("Quran", false),
                FilterOption::new("Seerah", false),
                FilterOption::new("Character", false),
                FilterOption::new("Family", false),
                FilterOption::new("Finance", false),
                FilterOption::new("Career", false),
                FilterOption::new("Study Skills", false),
                FilterOption::new("Technology", false),
                FilterOption::new("Community", false),
            ],
        ),
        FilterGroup::new(
            "Difficulty:",
            vec![
                FilterOption::new("All", true),
                FilterOption::new("Beginner", false),
                FilterOption::new("Intermediate", false),
                FilterOption::new("Advanced", false),
            ],
        ),
    ];

    view! {
        <div class="mr-4 space-y-12 py-8">
            <Filters
                search_placeholder="Search courses, lessons, topics, and educators...".to_string()
                filter_groups=filter_groups
            />

            <Suspense fallback=move || view! {
                <p class="text-sm text-foreground-600">"Loading your courses…"</p>
            }>
                {move || {
                    let courses = enrolled.get().unwrap_or_default();
                    view! { <ContinueLearningSection courses=courses /> }
                }}
            </Suspense>

            <section class="space-y-6">
                <div class="flex items-center justify-between gap-4">
                    <h1 class="text-3xl font-bold text-purple-900">"Featured Courses"</h1>
                    <A href="#" attr:class="font-medium text-purple-600 hover:text-purple-700">
                        "View All →"
                    </A>
                </div>

                <Suspense fallback=move || view! {
                    <p class="text-sm text-foreground-600">"Loading featured courses…"</p>
                }>
                    {move || {
                        let entries = catalog.get().unwrap_or_default();
                        view! { <FeaturedCoursesSection catalog=entries /> }
                    }}
                </Suspense>
            </section>

            <section class="space-y-6">
                <div class="flex items-center justify-between gap-4">
                    <h1 class="text-3xl font-bold text-purple-900">"Guided Learning Paths"</h1>
                    <A href="#" attr:class="font-medium text-purple-600 hover:text-purple-700">
                        "View All →"
                    </A>
                </div>

                <Suspense fallback=move || view! {
                    <p class="text-sm text-foreground-600">"Loading learning paths…"</p>
                }>
                    {move || {
                        let path_tracks = tracks.get().unwrap_or_default();
                        let path_catalog = catalog.get().unwrap_or_default();
                        let path_enrolled = enrolled.get().unwrap_or_default();
                        view! {
                            <LearningPathsSection
                                tracks=path_tracks
                                catalog=path_catalog
                                enrolled=path_enrolled
                            />
                        }
                    }}
                </Suspense>
            </section>

            <section class="space-y-6">
                <h1 class="text-3xl font-bold text-purple-900">"Recommended for You"</h1>

                <Suspense fallback=move || view! {
                    <p class="text-sm text-foreground-600">"Loading recommendations…"</p>
                }>
                    {move || {
                        let entries = catalog.get().unwrap_or_default();
                        view! { <RecommendedCoursesSection catalog=entries /> }
                    }}
                </Suspense>
            </section>
        </div>
    }
}
