use leptos::prelude::*;
use leptos_router::components::A;

use crate::components::cards::CourseCard;
use crate::components::filters::{FilterGroup, FilterOption, Filters, default_selection};
use crate::components::sections::ContinueLearningSection;
use crate::components::sections::FeaturedCoursesSection;
use crate::components::sections::LearningPathsSection;
use crate::components::sections::RecommendedCoursesSection;
use crate::components::sections::learn::recommended_courses_section::{
    category_class, format_duration, instructor_initials,
};
use crate::models::catalog_entry::CatalogEntry;
use crate::server_functions::education::fetch_my_courses;
use crate::server_functions::education::fetch_track_courses;
use crate::server_functions::education::fetch_tracks;
use crate::server_functions::education_roadmap::fetch_roadmaps;

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

    let roadmaps = Resource::new(|| (), |_| async move {
        fetch_roadmaps()
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

    let defaults = default_selection(&filter_groups);
    let default_category = defaults.first().cloned().unwrap_or_else(|| "All".to_string());
    let default_difficulty = defaults.get(1).cloned().unwrap_or_else(|| "All".to_string());
    let reset_category = default_category.clone();
    let reset_difficulty = default_difficulty.clone();
    let search = RwSignal::new(String::new());
    let active = RwSignal::new(defaults.clone());

    let searching = {
        let defaults = defaults.clone();
        move || {
            !search.get().trim().is_empty()
                || active.with(|selection| {
                    selection
                        .iter()
                        .zip(defaults.iter())
                        .any(|(current, default)| current != default)
                })
        }
    };

    let filtered_catalog = move || -> Vec<CatalogEntry> {
        let query = search.get().trim().to_lowercase();
        let selection = active.get();
        let category = selection.first().cloned().unwrap_or_else(|| "All".to_string());
        let difficulty = selection.get(1).cloned().unwrap_or_else(|| "All".to_string());

        catalog
            .get()
            .unwrap_or_default()
            .into_iter()
            .filter(|entry| {
                let course = &entry.course;
                let matches_query = query.is_empty()
                    || entry.track_name.to_lowercase().contains(&query)
                    || course.title.to_lowercase().contains(&query)
                    || course.short_description.to_lowercase().contains(&query)
                    || course.educator_name.to_lowercase().contains(&query);
                let matches_category = category == default_category
                    || entry.track_name.eq_ignore_ascii_case(&category);
                let matches_difficulty = difficulty == default_difficulty
                    || format!("{:?}", course.level).eq_ignore_ascii_case(&difficulty);

                matches_query && matches_category && matches_difficulty
            })
            .collect()
    };

    view! {
        <div class="min-w-0 space-y-8 py-6 sm:space-y-12 sm:py-8">
            <Filters
                search_placeholder="Search courses, lessons, topics, and educators...".to_string()
                filter_groups=filter_groups
                search=search
                active=active
            />

            {move || {
                if searching() {
                    let matches = filtered_catalog();
                    let count = matches.len();
                    let count_label = if count == 1 { "course" } else { "courses" };
                    let reset_selection = vec![reset_category.clone(), reset_difficulty.clone()];

                    view! {
                        <section class="min-w-0 space-y-6">
                            <div class="flex min-w-0 flex-wrap items-center justify-between gap-2">
                                <h1 class="min-w-0 text-2xl font-bold text-foreground-900 sm:text-3xl text-balance">"Search Results"</h1>
                                <span class="text-sm font-medium text-foreground-600">
                                    {format!("{count} {count_label} found")}
                                </span>
                            </div>

                            {if matches.is_empty() {
                                view! {
                                    <div class="min-w-0 rounded-2xl border border-dashed border-stroke-muted bg-surface-700 p-8 text-center shadow-sm">
                                        <p class="text-3xl" aria-hidden="true">"🔍"</p>
                                        <p class="mt-2 text-sm font-semibold text-foreground-900">
                                            "No courses match your search."
                                        </p>
                                        <p class="mt-1 text-sm text-foreground-600">
                                            "Try a different search term or clear the filters."
                                        </p>
                                        <button
                                            type="button"
                                            class="mt-4 inline-flex min-h-[44px] cursor-pointer items-center font-medium text-sky-600 dark:text-sky-400 hover:text-sky-700 dark:hover:text-sky-300"
                                            on:click=move |_| {
                                                search.set(String::new());
                                                active.set(reset_selection.clone());
                                            }
                                        >
                                            "Clear filters"
                                        </button>
                                    </div>
                                }
                                    .into_any()
                            } else {
                                view! {
                                    <div class="flex min-w-0 snap-x snap-mandatory gap-4 overflow-x-auto scrollbar-hide pb-4 sm:gap-6">
                                        {matches
                                            .into_iter()
                                            .enumerate()
                                            .map(|(index, entry)| {
                                                view! {
                                                    <CourseCard
                                                        category=entry.track_name.clone()
                                                        category_class=category_class(index).to_string()
                                                        badge=format!("{:?}", entry.course.level)
                                                        badge_class="bg-sky-100 dark:bg-sky-400/10 text-sky-700 dark:text-sky-400".to_string()
                                                        title=entry.course.title.clone()
                                                        description=entry.course.short_description.clone()
                                                        duration=format_duration(entry.course.duration_minutes)
                                                        lesson_count=format!("{} lessons", entry.course.lesson_count)
                                                        instructor_initials=instructor_initials(&entry.course.educator_name)
                                                        instructor_name=entry.course.educator_name.clone()
                                                        cta_label="Start Course".to_string()
                                                        cta_href=format!("/learn/courses/{}", entry.course.id)
                                                        img_link=entry
                                                            .course
                                                            .thumbnail_url
                                                            .clone()
                                                            .unwrap_or_else(|| {
                                                                "https://images.unsplash.com/photo-1481627834876-b7833e8f5570?auto=format&fit=crop&w=640&h=360&q=80"
                                                                    .to_string()
                                                            })
                                                    />
                                                }
                                            })
                                            .collect_view()}
                                    </div>
                                }
                                    .into_any()
                            }}
                        </section>
                    }
                        .into_any()
                } else {
                    view! {
                        <>
                            <Suspense fallback=move || view! {
                                <p class="text-sm text-foreground-600">"Loading your courses…"</p>
                            }>
                                {move || {
                                    let courses = enrolled.get().unwrap_or_default();
                                    view! { <ContinueLearningSection courses=courses /> }
                                }}
                            </Suspense>

                            <section class="min-w-0 space-y-6">
                                <div class="flex min-w-0 flex-wrap items-center justify-between gap-2">
                                    <h1 class="min-w-0 text-2xl font-bold text-foreground-900 sm:text-3xl text-balance">"Featured Courses"</h1>
                                    <A href="/learn/courses" attr:class="inline-flex min-h-[44px] shrink-0 items-center whitespace-nowrap font-medium text-sky-600 dark:text-sky-400 hover:text-sky-700 dark:hover:text-sky-300">
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

                            <section class="min-w-0 space-y-6">
                                <div class="flex min-w-0 flex-wrap items-center justify-between gap-2">
                                    <h1 class="min-w-0 text-2xl font-bold text-foreground-900 sm:text-3xl text-balance">"Guided Learning Paths"</h1>
                                    <A href="/learn/paths" attr:class="inline-flex min-h-[44px] shrink-0 items-center whitespace-nowrap font-medium text-sky-600 dark:text-sky-400 hover:text-sky-700 dark:hover:text-sky-300">
                                        "View All →"
                                    </A>
                                </div>

                                <Suspense fallback=move || view! {
                                    <p class="text-sm text-foreground-600">"Loading learning paths…"</p>
                                }>
                                    {move || {
                                        let path_roadmaps = roadmaps.get().unwrap_or_default();
                                        let path_enrolled = enrolled.get().unwrap_or_default();
                                        view! {
                                            <LearningPathsSection roadmaps=path_roadmaps enrolled=path_enrolled />
                                        }
                                    }}
                                </Suspense>
                            </section>

                            <section class="min-w-0 space-y-6">
                                <h1 class="min-w-0 text-2xl font-bold text-foreground-900 sm:text-3xl text-balance">"Recommended for You"</h1>

                                <Suspense fallback=move || view! {
                                    <p class="text-sm text-foreground-600">"Loading recommendations…"</p>
                                }>
                                    {move || {
                                        let entries = catalog.get().unwrap_or_default();
                                        view! { <RecommendedCoursesSection catalog=entries /> }
                                    }}
                                </Suspense>
                            </section>
                        </>
                    }
                        .into_any()
                }
            }}
        </div>
    }
}
