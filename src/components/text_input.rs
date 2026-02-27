use leptos::{html, prelude::*};

#[component]
pub fn TextInput(
    label: &'static str,
    name: &'static str,
    placeholder: &'static str,
    input_type: &'static str,
    node_ref: NodeRef<html::Input>,
    error_signal: ReadSignal<String>,
    hint: &'static str,
) -> impl IntoView {
    view! {
        <div class = "form-group grid">
            <label for = {name} class = "">{label}</label>
            <input
                type = {input_type}
                name = {name}
                placeholder = {placeholder}
                node_ref = node_ref
                required
                class = "border-[0.125rem] border-stroke outline-none transition-all focus:border-indigo-300 focus:ring-4 focus:ring-indigo-400/20 rounded-2xl mt-1 mb-0.5 py-2 px-3 bg-surface-750"
            />
            <Show when = move || !error_signal.get().is_empty()>
                <p class = "text-danger text-[.9rem]">{error_signal.get()}</p>
            </Show>
            <Show when = move || error_signal.get().is_empty()>
                <p class = "text-foreground-600 text-[.9rem]">{hint}</p>
            </Show>
        </div>
    }
}
