use crate::app::AppState;
use crate::app::AppStateStoreFields;
use crate::models::api_responses::MixedMosqueResponse;
use crate::server_functions::mosque::fetch_mosques_for_location;
use crate::server_functions::mosque::get_favorite_mosque;
use leptos::IntoView;
use leptos::prelude::*;
use reactive_stores::OptionStoreExt;
use reactive_stores::Store;

#[component]
pub fn NearbyMosquesSection() -> impl IntoView {
    let app_state = expect_context::<Store<AppState>>();
    let user_pos = move || app_state.coords().map(|coords| coords.read().as_lat_lon());

    let mosques = Resource::new(user_pos, move |pos: Option<(f64, f64)>| async move {
        match pos {
            Some((lat, lon)) => fetch_mosques_for_location(lat, lon, None)
                .await
                .ok()
                .and_then(|res| res.data)
                .and_then(|mosques| match mosques {
                    MixedMosqueResponse::SingleMosque(mosque) => Some(vec![mosque]),
                    MixedMosqueResponse::MosquesVec(mosques) => Some(mosques),
                }),
            None => get_favorite_mosque(None, None)
                .await
                .ok()
                .and_then(|res| res.data)
                .and_then(|fav| match fav {
                    MixedMosqueResponse::MosquesVec(mosques) => Some(mosques),
                    MixedMosqueResponse::SingleMosque(mosque) => Some(vec![mosque]),
                }),
        }
    });

    view! {
        <div class="flex gap-5 overflow-x-scroll pb-4"></div>
    }
}
