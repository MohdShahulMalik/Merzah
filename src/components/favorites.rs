//! Shared state for the user's favorite ("liked") mosques.
//!
//! Favorites are fetched once when the context is provided and kept in a
//! signal so every mosque card / modal reflects the same state. Toggling is
//! optimistic: the signal is updated immediately and reverted when the
//! server call fails.

use std::collections::HashSet;

use leptos::prelude::*;

use crate::models::api_responses::MixedMosqueResponse;
use crate::server_functions::mosque::{add_favorite, get_favorite_mosque, remove_favorite};

/// Context holding the ids of the mosques the current user has favorited.
#[derive(Clone, Copy)]
pub struct FavoritesContext {
    favorite_ids: RwSignal<HashSet<String>>,
    /// The latest favorites failure, shown as a dismissible toast. Cleared
    /// on the next successful toggle.
    error: RwSignal<Option<String>>,
}

impl FavoritesContext {
    pub fn error(&self) -> RwSignal<Option<String>> {
        self.error
    }
    fn new() -> Self {
        let favorite_ids = RwSignal::new(HashSet::new());
        let error = RwSignal::new(None);

        // Load the user's favorites once, on the client. Unauthenticated
        // users (or any failure) simply end up with an empty set.
        Effect::new(move |_| {
            leptos::task::spawn_local(async move {
                let favorites = match get_favorite_mosque(None, None).await {
                    Ok(response) => response.data.map(|mixed| match mixed {
                        MixedMosqueResponse::SingleMosque(mosque) => vec![mosque.id],
                        MixedMosqueResponse::MosquesVec(mosques) => {
                            mosques.into_iter().map(|mosque| mosque.id).collect()
                        }
                    }),
                    Err(_) => None,
                };

                if let Some(ids) = favorites {
                    favorite_ids.set(ids.into_iter().collect());
                }
            });
        });

        Self { favorite_ids, error }
    }

    /// Reactively checks whether the mosque is favorited.
    pub fn is_favorite(&self, mosque_id: &str) -> bool {
        self.favorite_ids.with(|ids| ids.contains(mosque_id))
    }

    /// Toggles the favorite state of a mosque, updating the UI immediately
    /// and reverting when the server request fails.
    pub fn toggle_favorite(&self, mosque_id: String) {
        let favorite_ids = self.favorite_ids;
        let error = self.error;
        let was_favorite = favorite_ids.get_untracked().contains(&mosque_id);

        favorite_ids.update(|ids| {
            if was_favorite {
                ids.remove(&mosque_id);
            } else {
                ids.insert(mosque_id.clone());
            }
        });

        leptos::task::spawn_local(async move {
            let result = if was_favorite {
                remove_favorite(mosque_id.clone()).await
            } else {
                add_favorite(mosque_id.clone()).await
            };

            match result {
                Ok(response) => {
                    if let Some(message) = response.error {
                        favorite_ids.update(|ids| {
                            if was_favorite {
                                ids.insert(mosque_id.clone());
                            } else {
                                ids.remove(&mosque_id);
                            }
                        });
                        leptos::logging::log!("Favorite toggle rejected by the server: {message}");
                        error.set(Some(format!(
                            "Couldn't update favorites: {message}"
                        )));
                    } else {
                        error.set(None);
                    }
                }
                Err(e) => {
                    favorite_ids.update(|ids| {
                        if was_favorite {
                            ids.insert(mosque_id.clone());
                        } else {
                            ids.remove(&mosque_id);
                        }
                    });
                    leptos::logging::error!("Favorite toggle failed: {e}");
                    error.set(Some(
                        "Couldn't update favorites. Please log in again and retry."
                            .to_string(),
                    ));
                }
            }
        });
    }
}

/// Provides the favorites context. Call once, high in the tree.
pub fn provide_favorites() {
    provide_context(FavoritesContext::new());
}

/// Returns the shared favorites context.
pub fn use_favorites() -> FavoritesContext {
    expect_context::<FavoritesContext>()
}

/// A small dismissible toast shown when a favorite toggle fails on the
/// server, so the silent revert doesn't leave the user guessing.
#[component]
pub fn FavoritesErrorToast() -> impl IntoView {
    let favorites = use_favorites();
    let error = favorites.error();

    view! {
        <Show when=move || error.get().is_some()>
            <div
                role="alert"
                class="fixed bottom-4 left-1/2 z-50 flex w-[calc(100vw-2rem)] max-w-md -translate-x-1/2 items-center gap-3 rounded-2xl border border-red-400/40 bg-surface-700 px-4 py-3 shadow-2xl"
            >
                <svg
                    class="h-5 w-5 shrink-0 text-red-400"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="1.8"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    aria-hidden="true"
                >
                    <circle cx="12" cy="12" r="9" />
                    <path d="M12 8v4" />
                    <path d="M12 16h.01" />
                </svg>
                <p class="min-w-0 flex-1 text-sm text-foreground-900">{move || error.get().unwrap_or_default()}</p>
                <button
                    type="button"
                    class="grid h-9 w-9 shrink-0 cursor-pointer place-items-center rounded-lg text-foreground-600 transition-colors duration-200 hover:bg-surface-600 hover:text-foreground-900"
                    aria-label="Dismiss"
                    on:click=move |_| error.set(None)
                >
                    <svg
                        class="h-4 w-4"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        aria-hidden="true"
                    >
                        <path d="M18 6 6 18" />
                        <path d="m6 6 12 12" />
                    </svg>
                </button>
            </div>
        </Show>
    }
}
