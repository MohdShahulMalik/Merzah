use leptos::prelude::*;
use leptos_leaflet::prelude::{Circle, MapContainer, Marker, Popup, Position, TileLayer, Tooltip};
use leptos_meta::Stylesheet;
use reactive_stores::Store;

use crate::app::AppState;
use crate::app::AppStateStoreFields;
use crate::models::api_responses::MixedMosqueResponse;
use crate::models::api_responses::MosqueResponse;
use crate::server_functions::mosque::fetch_mosques_for_location;
use reactive_stores::OptionStoreExt;

const DEFAULT_ZOOM: f64 = 14.0;
const FALLBACK_CENTER: (f64, f64) = (21.4225, 39.8262);

#[component]
pub fn Mosques() -> impl IntoView {
    let app_state = expect_context::<Store<AppState>>();
    let user_pos = move || {
        app_state
            .coords()
            .map(|coords| coords.read().as_lat_lon())
    };

    let mosques = Resource::new(user_pos, |pos: Option<(f64, f64)>| async move {
        match pos {
            Some((lat, lon)) => fetch_mosques_for_location(lat, lon, false)
                .await
                .ok()
                .and_then(|res| res.data)
                .map(|mixed| match mixed {
                    MixedMosqueResponse::SingleMosque(mosque) => vec![mosque],
                    MixedMosqueResponse::MosquesVec(mosques) => mosques,
                })
                .unwrap_or_default(),
            None => Vec::new(),
        }
    });

    let center = Memo::new(move |_| match user_pos() {
        Some((lat, lon)) => Position::new(lat, lon),
        None => Position::new(FALLBACK_CENTER.0, FALLBACK_CENTER.1),
    });

    let has_pos = Memo::new(move |_| user_pos().is_some());

    view! {
        <Stylesheet href="https://unpkg.com/leaflet@1.9.4/dist/leaflet.css" />
        <div class="space-y-4 mt-4 mr-4 mb-4">
            <div class="flex items-center justify-between">
                <h1 class="text-2xl font-bold text-purple-900">"Nearby Mosques Map"</h1>
                <p class="text-sm text-gray-500">
                    {move || {
                        let count = mosques.get().map(|m| m.len()).unwrap_or(0);
                        format!("{} mosques nearby", count)
                    }}
                </p>
            </div>

            <Show
                when=move || has_pos.get()
                fallback=|| {
                    view! {
                        <div class="rounded-2xl border border-gray-200 bg-white p-6 text-center shadow-sm">
                            <p class="text-gray-700 font-medium">"Locating you…"</p>
                            <p class="mt-1 text-sm text-gray-500">
                                "Please allow location access to see mosques around you on the map."
                            </p>
                        </div>
                    }
                }
            >
                {move || {
                    let map_center = center.get();
                    view! {
                        <div class="overflow-hidden rounded-2xl border border-gray-200 shadow-sm">
                            <MapContainer
                                style="height: 65vh; width: 100%; z-index: 0;"
                                center=map_center
                                zoom=DEFAULT_ZOOM
                                set_view=true
                            >
                                <TileLayer
                                    url="https://tile.openstreetmap.org/{z}/{x}/{y}.png"
                                    attribution="© <a href=\"https://www.openstreetmap.org/copyright\">OpenStreetMap</a> contributors"
                                />
                                // User location marker + accuracy circle.
                                {move || {
                                    user_pos()
                                        .map(|(lat, lon)| {
                                            let user_position = Position::new(lat, lon);
                                            view! {
                                                <Circle
                                                    center=user_position
                                                    radius=300.0
                                                    color="blue"
                                                />
                                                <Marker position=user_position>
                                                    <Popup>"You are here"</Popup>
                                                    <Tooltip permanent=true direction="top">
                                                        "You are here"
                                                    </Tooltip>
                                                </Marker>
                                            }
                                        })
                                }}
                                // Mosque markers fetched for the user's location.
                                {move || {
                                    mosques
                                        .get()
                                        .unwrap_or_default()
                                        .into_iter()
                                        .map(|mosque: MosqueResponse| {
                                            let position = Position::new(
                                                mosque.location.0,
                                                mosque.location.1,
                                            );
                                            let name = mosque
                                                .name
                                                .clone()
                                                .unwrap_or_else(|| "Mosque".to_string());
                                            let detail = mosque
                                                .street
                                                .clone()
                                                .or(mosque.city.clone())
                                                .unwrap_or_default();
                                            let popup_name = name.clone();
                                            let tooltip_name = name.clone();
                                            let popup_detail = detail.clone();
                                            view! {
                                                <Marker position=position>
                                                    <Popup>
                                                        <strong>{popup_name}</strong>
                                                        <br />
                                                        <span>{popup_detail}</span>
                                                    </Popup>
                                                    <Tooltip direction="top">{tooltip_name}</Tooltip>
                                                </Marker>
                                            }
                                        })
                                        .collect::<Vec<_>>()
                                }}
                            </MapContainer>
                        </div>
                    }
                }}
            </Show>
        </div>
    }
}
