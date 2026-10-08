use leptos::IntoView;
use leptos::prelude::*;

use crate::components::cards::CourseCard;
use crate::models::catalog_entry::CatalogEntry;

const FEATURED_COUNT: usize = 4;

fn category_class(index: usize) -> &'static str {
    match index % 6 {
        0 => "bg-green-100 text-green-700",
        1 => "bg-blue-100 text-blue-700",
        2 => "bg-amber-100 text-amber-700",
        3 => "bg-rose-100 text-rose-700",
        4 => "bg-emerald-100 text-emerald-700",
        _ => "bg-orange-100 text-orange-700",
    }
}

fn format_duration(minutes: i32) -> String {
    let minutes = minutes.max(0);
    let hours = minutes / 60;
    let rest = minutes % 60;

    if hours == 0 {
        format!("{rest} min")
    } else if rest == 0 {
        format!("{hours} hours")
    } else {
        format!("{hours}h {rest}m")
    }
}

fn instructor_initials(name: &str) -> String {
    let mut parts = name.split_whitespace();
    let first = parts.next().and_then(|word| word.chars().next());
    let second = parts.next().and_then(|word| word.chars().next());

    match (first, second) {
        (Some(first), Some(second)) => format!("{first}{second}").to_uppercase(),
        (Some(first), None) => format!("{first}").to_uppercase(),
        _ => "?".to_string(),
    }
}

#[component]
pub fn FeaturedCoursesSection(catalog: Memo<Vec<CatalogEntry>>) -> impl IntoView {
    view! {
        <div class="flex gap-6 w-full overflow-x-scroll scrollbar-hide pb-4">
            <ForEnumerate
                each=move || {
                    catalog.get().into_iter().take(FEATURED_COUNT).collect::<Vec<_>>()
                }
                key=|entry| entry.course.id.clone()
                let(idx, entry)
            >
                <CourseCard
                    category=entry.track_name.clone()
                    category_class=category_class(idx.get()).to_string()
                    badge=format!("{:?}", entry.course.level)
                    badge_class="bg-purple-100 text-purple-700".to_string()
                    title=entry.course.title.clone()
                    description=entry.course.short_description.clone()
                    duration=format_duration(entry.course.duration_minutes)
                    lesson_count=format!("{} lessons", entry.course.lesson_count)
                    instructor_initials=instructor_initials(&entry.course.educator_name)
                    instructor_name=entry.course.educator_name.clone()
                    cta_label="Start Course".to_string()
                    img_link=entry
                        .course
                        .thumbnail_url
                        .clone()
                        .unwrap_or_else(|| {
                            "https://images.unsplash.com/photo-1481627834876-b7833e8f5570?auto=format&fit=crop&w=640&h=360&q=80"
                                .to_string()
                        })
                />
            </ForEnumerate>
            <Show when=move || catalog.get().is_empty()>
                <p class="text-sm text-foreground-600">"No featured courses yet."</p>
            </Show>
        </div>
    }
}
