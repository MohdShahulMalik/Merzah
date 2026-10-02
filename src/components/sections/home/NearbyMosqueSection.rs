use crate::app::AppState;
use crate::app::AppStateStoreFields;
use leptos::IntoView;
use leptos::prelude::*;
use reactive_stores::OptionStoreExt;
use reactive_stores::Store;

#[component]
pub fn NearbyMosquesSection() -> impl IntoView {
    let app_state = expect_context::<Store<AppState>>();
    let _user_pos = move || app_state.coords().map(|coords| coords.read().as_lat_lon());

    view! {
        <div class="flex gap-5 overflow-x-scroll pb-4"></div>
    }
}
