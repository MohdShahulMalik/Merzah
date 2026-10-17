use leptos::prelude::*;

#[derive(Clone, Debug)]
pub struct FilterOption {
    pub label: String,
    pub is_active: bool,
}

impl FilterOption {
    pub fn new(label: impl Into<String>, is_active: bool) -> Self {
        Self {
            label: label.into(),
            is_active,
        }
    }
}

#[derive(Clone, Debug)]
pub struct FilterGroup {
    pub label: String,
    pub options: Vec<FilterOption>,
}

impl FilterGroup {
    pub fn new(label: impl Into<String>, options: Vec<FilterOption>) -> Self {
        Self {
            label: label.into(),
            options,
        }
    }
}

#[component]
pub fn Filters(
    search_placeholder: String,
    #[prop(optional)] location_label: Option<String>,
    filter_groups: Vec<FilterGroup>,
) -> impl IntoView {
    view! {
        <section class="min-w-0 rounded-2xl border border-stroke-muted bg-surface-700 p-4 shadow-[0_0.75rem_2rem_rgba(31,20,58,0.08)] sm:p-5 md:p-7">
            <div class="flex min-w-0 flex-col gap-3 md:flex-row">
                <label class="flex min-h-[44px] min-w-0 flex-1 items-center gap-3 rounded-xl border border-stroke-muted bg-surface-700 px-4 text-foreground-600 focus-within:border-primary focus-within:ring-2 focus-within:ring-primary/15">
                    <span class="text-lg" aria-hidden="true">"🔍"</span>
                    <input
                        type="search"
                        placeholder=search_placeholder
                        class="w-full min-w-0 border-0 bg-transparent text-base text-foreground-900 outline-none placeholder:text-foreground-600"
                    />
                </label>

                {location_label.map(|location_label| {
                    view! {
                        <button class="inline-flex min-h-[44px] min-w-0 items-center justify-center gap-3 rounded-xl border border-stroke-muted bg-surface-750 px-5 text-base font-medium text-foreground-900 transition-colors hover:bg-surface-900 md:min-w-44">
                            <span aria-hidden="true">"📍"</span>
                            <span class="truncate">{location_label}</span>
                        </button>
                    }
                })}
            </div>

            <div class="mt-5 min-w-0 divide-y divide-violet-950/10">
                {filter_groups
                    .into_iter()
                    .map(|group| {
                        view! {
                            <div class="flex min-w-0 flex-col gap-3 py-5 first:pt-0 last:pb-0 sm:flex-row sm:items-center">
                                <p class="min-w-24 shrink-0 text-sm font-bold text-foreground-600">{group.label}</p>
                                <div class="flex min-w-0 flex-wrap gap-2.5">
                                    {group.options
                                        .into_iter()
                                        .map(|option| {
                                            let option_class = if option.is_active {
                                                "bg-primary text-white border-primary shadow-sm"
                                            } else {
                                                "bg-surface-750 text-foreground-900 border-stroke-muted hover:border-primary/35 hover:bg-surface-900"
                                            };

                                            view! {
                                                <button class=format!("inline-flex min-h-[44px] items-center rounded-full border px-5 py-2 text-sm font-medium transition-colors {option_class}")>
                                                    {option.label}
                                                </button>
                                            }
                                        })
                                        .collect::<Vec<_>>()}
                                </div>
                            </div>
                        }
                    })
                    .collect::<Vec<_>>()}
            </div>
        </section>
    }
}
