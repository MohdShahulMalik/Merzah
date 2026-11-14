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

    /// The label of the option marked as active, falling back to the first
    /// option. Used to initialise the controlled `active` selection held by
    /// the page.
    pub fn default_label(&self) -> String {
        self.options
            .iter()
            .find(|option| option.is_active)
            .or_else(|| self.options.first())
            .map(|option| option.label.clone())
            .unwrap_or_default()
    }
}

/// Builds the default selection (one label per group) from the groups' active
/// options. Pages use this to initialise the `active` signal they pass in.
pub fn default_selection(filter_groups: &[FilterGroup]) -> Vec<String> {
    filter_groups
        .iter()
        .map(FilterGroup::default_label)
        .collect()
}

/// A controlled search + filter bar. The page owns the `search` query and the
/// `active` selection (one label per group, in the same order as
/// `filter_groups`) and decides what to show based on them; this component
/// only renders the controls and reports changes back through the signals.
#[component]
pub fn Filters(
    search_placeholder: String,
    #[prop(optional)] location_label: Option<Signal<String>>,
    filter_groups: Vec<FilterGroup>,
    search: RwSignal<String>,
    active: RwSignal<Vec<String>>,
) -> impl IntoView {
    view! {
        <section class="min-w-0 rounded-2xl border border-stroke-muted bg-surface-700 p-4 shadow-[0_0.75rem_2rem_rgba(31,20,58,0.08)] sm:p-5 md:p-7">
            <div class="flex min-w-0 flex-col gap-3 md:flex-row">
                <label class="flex min-h-[44px] min-w-0 flex-1 items-center gap-3 rounded-xl border border-stroke-muted bg-surface-750 px-4 text-foreground-600 focus-within:border-primary focus-within:ring-2 focus-within:ring-primary/15">
                    <span class="text-lg" aria-hidden="true">"🔍"</span>
                    <input
                        type="search"
                        placeholder=search_placeholder
                        prop:value=move || search.get()
                        on:input=move |ev| search.set(event_target_value(&ev))
                        class="w-full min-w-0 border-0 bg-transparent text-base text-foreground-900 outline-none placeholder:text-foreground-600"
                    />
                </label>

                {location_label.map(|location_label| {
                    view! {
                        <div class="inline-flex min-h-[44px] min-w-0 items-center justify-center gap-3 rounded-xl border border-stroke-muted bg-surface-750 px-5 text-base font-medium text-foreground-900 select-none md:min-w-44">
                            <span aria-hidden="true">"📍"</span>
                            <span class="truncate">{move || location_label.get()}</span>
                        </div>
                    }
                })}
            </div>

            <div class="mt-5 min-w-0 divide-y divide-violet-950/10 dark:divide-white/10">
                {filter_groups
                    .into_iter()
                    .enumerate()
                    .map(|(group_index, group)| {
                        view! {
                            <div class="flex min-w-0 flex-col gap-3 py-5 first:pt-0 last:pb-0 sm:flex-row sm:items-center">
                                <p class="min-w-24 shrink-0 text-sm font-bold text-foreground-600">{group.label}</p>
                                <div class="flex min-w-0 flex-nowrap gap-2.5 overflow-x-auto scrollbar-hide">
                                    {group.options
                                        .into_iter()
                                        .map(|option| {
                                            let label = option.label;
                                            let active_label = label.clone();
                                            let click_label = label.clone();
                                            let is_active = move || {
                                                active.with(|selection| {
                                                    selection
                                                        .get(group_index)
                                                        .is_some_and(|current| *current == active_label)
                                                })
                                            };

                                            view! {
                                                <button
                                                    class=move || {
                                                        let option_class = if is_active() {
                                                            "bg-primary text-white dark:text-surface-900 border-primary shadow-sm"
                                                        } else {
                                                            "bg-surface-750 text-foreground-900 border-stroke-muted hover:border-primary/35 hover:bg-surface-900"
                                                        };

                                                        format!("inline-flex min-h-[44px] shrink-0 items-center rounded-2xl border px-5 py-2 text-sm font-medium transition-colors {option_class}")
                                                    }
                                                    on:click=move |_| {
                                                        active.update(|selection| {
                                                            if let Some(slot) = selection.get_mut(group_index) {
                                                                *slot = click_label.clone();
                                                            }
                                                        });
                                                    }
                                                >
                                                    {label}
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
