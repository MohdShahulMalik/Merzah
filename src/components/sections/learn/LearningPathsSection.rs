use std::collections::HashSet;

use leptos::IntoView;
use leptos::prelude::*;

use crate::components::cards::LearningPathCard;
use crate::models::catalog_entry::CatalogEntry;
use crate::models::education::EnrollmentProgress;
use crate::models::education::TrackOnClient;

fn style(index: usize) -> (&'static str, &'static str, &'static str) {
    match index % 4 {
        0 => ("text-purple-900", "text-purple-500", "text-purple-200"),
        1 => ("text-indigo-900", "text-indigo-500", "text-indigo-200"),
        2 => ("text-teal-900", "text-teal-500", "text-teal-200"),
        _ => ("text-amber-900", "text-amber-500", "text-amber-200"),
    }
}

fn format_duration(minutes: i32) -> String {
    let minutes = minutes.max(0);
    let hours = minutes / 60;
    let rest = minutes % 60;

    if minutes == 0 {
        "--".to_string()
    } else if hours == 0 {
        format!("{rest} min")
    } else if rest == 0 {
        format!("{hours} hours")
    } else {
        format!("{hours}h {rest}m")
    }
}

#[component]
pub fn LearningPathsSection(
    tracks: Memo<Vec<TrackOnClient>>,
    catalog: Memo<Vec<CatalogEntry>>,
    enrolled: Memo<Vec<EnrollmentProgress>>,
) -> impl IntoView {
    view! {
        <div class="grid gap-6 md:grid-cols-2">
            <ForEnumerate
                each=move || tracks.get()
                key=|track| track.id.clone()
                let(idx, track)
            >
                {
                    let (title_class, description_class, icon_class) = style(idx.get());
                    let catalog_items = catalog.get();
                    let enrolled_items = enrolled.get();
                    let course_ids: HashSet<String> = catalog_items
                        .iter()
                        .filter(|entry| entry.track_id == track.id)
                        .map(|entry| entry.course.id.clone())
                        .collect();
                    let total_minutes: i32 = catalog_items
                        .iter()
                        .filter(|entry| entry.track_id == track.id)
                        .map(|entry| entry.course.duration_minutes)
                        .sum();
                    let track_progress: Vec<f32> = enrolled_items
                        .iter()
                        .filter(|progress| course_ids.contains(&progress.course_id))
                        .map(|progress| progress.progress_percent)
                        .collect();
                    let average = if track_progress.is_empty() {
                        None
                    } else {
                        Some(
                            track_progress.iter().sum::<f32>()
                                / track_progress.len() as f32,
                        )
                    };

                    view! {
                        <LearningPathCard
                            title=track.name.clone()
                            description=track.description.clone()
                            title_class=title_class.to_string()
                            description_class=description_class.to_string()
                            icon=track.icon.clone().unwrap_or_else(|| "📚".to_string())
                            icon_class=icon_class.to_string()
                            course_count=format!("{} courses", track.course_count)
                            duration=format_duration(total_minutes)
                            progress_label=match average {
                                Some(percent) => format!("{percent:.0}%"),
                                None => "Not Started".to_string(),
                            }
                            progress_percent=match average {
                                Some(percent) => percent.clamp(0.0, 100.0).round() as u8,
                                None => 0,
                            }
                            cta_label=if average.is_some() {
                                "Continue Path".to_string()
                            } else {
                                "Begin Path".to_string()
                            }
                        />
                    }
                }
            </ForEnumerate>
            <Show when=move || tracks.get().is_empty()>
                <p class="text-sm text-foreground-600">"No learning paths yet."</p>
            </Show>
        </div>
    }
}
