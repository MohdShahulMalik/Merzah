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
        <section class="rounded-2xl border border-violet-950/10 bg-white p-5 shadow-[0_0.75rem_2rem_rgba(31,20,58,0.08)] md:p-7">
            <div class="flex flex-col gap-3 md:flex-row">
                <label class="flex min-h-14 flex-1 items-center gap-3 rounded-xl border border-violet-950/15 bg-white px-4 text-foreground-600 focus-within:border-primary focus-within:ring-2 focus-within:ring-primary/15">
                    <span class="text-lg" aria-hidden="true">"🔍"</span>
                    <input
                        type="search"
                        placeholder=search_placeholder
                        class="w-full border-0 bg-transparent text-base text-foreground-900 outline-none placeholder:text-foreground-600"
                    />
                </label>

                {location_label.map(|location_label| {
                    view! {
                        <button class="inline-flex min-h-14 items-center justify-center gap-3 rounded-xl border border-violet-950/15 bg-surface-750 px-5 text-base font-medium text-foreground-900 transition-colors hover:bg-surface-900 md:min-w-44">
                            <span aria-hidden="true">"📍"</span>
                            <span>{location_label}</span>
                        </button>
                    }
                })}
            </div>

            <div class="mt-5 divide-y divide-violet-950/10">
                {filter_groups
                    .into_iter()
                    .map(|group| {
                        view! {
                            <div class="flex flex-col gap-3 py-5 first:pt-0 last:pb-0 sm:flex-row sm:items-center">
                                <p class="min-w-24 text-sm font-bold text-foreground-600">{group.label}</p>
                                <div class="flex flex-wrap gap-2.5">
                                    {group.options
                                        .into_iter()
                                        .map(|option| {
                                            let option_class = if option.is_active {
                                                "bg-primary text-white border-primary shadow-sm"
                                            } else {
                                                "bg-surface-750 text-foreground-900 border-violet-950/10 hover:border-primary/35 hover:bg-surface-900"
                                            };

                                            view! {
                                                <button class=format!("rounded-full border px-5 py-2 text-sm font-medium transition-colors {option_class}")>
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
