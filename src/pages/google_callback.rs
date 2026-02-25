use leptos::{prelude::*, reactive::spawn_local};
use leptos_router::hooks::use_query_map;
use crate::server_functions::auth::handle_google_callback;

#[component]
pub fn GoogleCallback() -> impl IntoView {
    let query = use_query_map();
    let (error, set_error) = signal(String::new());
    let (success, set_success) = signal(false);
    let (loading, set_loading) = signal(true);

    let handle_callback = move || {
        let q = query.get();
        let code = q.get("code").unwrap_or_default();
        let state = q.get("state").unwrap_or_default();

        if code.is_empty() {
            set_loading.set(false);
            set_error.set("No authorization code found.".to_string());
            return;
        }

        spawn_local(async move {
            match handle_google_callback(code, state).await {
                Ok(response) => {
                    if let Some(err_msg) = response.error {
                        set_error.set(err_msg);
                    } else {
                        set_success.set(true);
                        // TODO: Use better approach to redirect after successful authentication if possible.
                        let _ = window().location().set_href("/dashboard");
                    }
                }
                Err(e) => {
                    set_error.set(e.to_string());
                }
            }
            set_loading.set(false);
        });
    };

    Effect::new(move |_| {
        query.get();
        if loading.get() {
            handle_callback();
        }
    });

    view! {
        <div class="flex items-center justify-center min-h-screen">
            <div class="text-center">
                <Show when=move || !loading.get() fallback=move || view! {
                    <p>Authenticating with Google...</p>
                }>
                    <Show when=move || error.get().is_empty() fallback=move || view! {
                        <div class="text-red-500">
                            <p>{error.get()}</p>
                            <a href="/login" class="text-blue-500 underline">Try again</a>
                        </div>
                    }>
                        <Show when=move || success.get() fallback=move || view! {{}}>
                            <p class="text-green-500">Successfully authenticated! Redirecting...</p>
                        </Show>
                    </Show>
                </Show>
            </div>
        </div>
    }
}
