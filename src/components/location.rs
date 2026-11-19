use leptos::IntoView;
use leptos::prelude::*;
use leptos_use::UseGeolocationReturn;
use leptos_use::storage::use_local_storage;
use leptos_use::use_geolocation;
use reactive_stores::Store;
use codee::string::JsonSerdeCodec;

use crate::app::AppState;
use crate::app::AppStateStoreFields;
use crate::models::location::GeoPosition;

/// Watches the device location once and mirrors it into the global [`AppState`].
///
/// `use_geolocation` returns a local (non-`Send`) signal, so coordinates are
/// copied into the app store, which any descendant can read via context.
/// The last known position is persisted to local storage so it can be restored
/// before the browser grants (or while it re-prompts for) geolocation permission.
#[component]
pub fn LocationProvider() -> impl IntoView {
    let app_state = expect_context::<Store<AppState>>();
    let initialized = StoredValue::new(false);

    Effect::new(move |_| {
        if app_state.user().get().is_none() {
            return;
        }
        if initialized.get_value() {
            return;
        }
        initialized.set_value(true);

        let (stored, set_stored, _) =
            use_local_storage::<Option<GeoPosition>, JsonSerdeCodec>("geo-position");

        if app_state.coords().get_untracked().is_none() {
            let cached = stored.get_untracked();
            if cached.is_some() {
                app_state.coords().set(cached);
            }
        }

        // `coords` is a local (non-`Send`) signal created once per user; a
        // separate inner effect mirrors it into the store and local storage.
        let UseGeolocationReturn { coords, .. } = use_geolocation();
        Effect::new(move |_| {
            if let Some(coords) = coords.get() {
                let position = GeoPosition::new(coords.latitude(), coords.longitude());
                app_state.coords().set(Some(position));
                set_stored.set(Some(position));
            }
        });
    });

    view! {}
}
