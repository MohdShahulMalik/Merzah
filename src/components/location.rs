use leptos::IntoView;
use leptos::prelude::*;
use leptos_use::UseGeolocationReturn;
use leptos_use::use_geolocation;
use reactive_stores::Store;

use crate::app::AppState;
use crate::app::AppStateStoreFields;
use crate::models::location::GeoPosition;

/// Watches the device location once and mirrors it into the global [`AppState`].
///
/// `use_geolocation` returns a local (non-`Send`) signal, so coordinates are
/// copied into the app store, which any descendant can read via context.
#[component]
pub fn LocationProvider() -> impl IntoView {
    let app_state = expect_context::<Store<AppState>>();
    let UseGeolocationReturn { coords, .. } = use_geolocation();

    Effect::new(move |_| {
        let position = coords
            .get()
            .map(|coords| GeoPosition::new(coords.latitude(), coords.longitude()));

        app_state.coords().set(position);
    });

    view! {}
}
