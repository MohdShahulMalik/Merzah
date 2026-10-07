use chrono::Datelike;
use chrono::Local;
use chrono::Weekday;
use leptos::IntoView;
use leptos::prelude::*;
use reactive_stores::OptionStoreExt;
use reactive_stores::Store;

use crate::app::AppState;
use crate::app::AppStateStoreFields;
use crate::components::cards::NearbyMosqueCard;
use crate::models::api_responses::MixedMosqueResponse;
use crate::models::nearby_mosque::NearbyMosqueCardItem;
use crate::server_functions::mosque::fetch_mosques_for_location;
use crate::server_functions::mosque::get_favorite_mosque;
use crate::utils::prayer::format_iqamah;
use crate::utils::prayer::next_jamat;

const PREVIEW_SIZE: usize = 5;

#[component]
pub fn NearbyMosquesSection(show_all: ReadSignal<bool>) -> impl IntoView {
    let app_state = expect_context::<Store<AppState>>();
    let user_pos = move || {
        app_state
            .coords()
            .map(|coords| coords.read().as_lat_lon())
    };

    let mosques = Resource::new(user_pos, move |pos: Option<(f64, f64)>| async move {
        match pos {
            Some((lat, lon)) => fetch_mosques_for_location(lat, lon, false)
                .await
                .ok()
                .and_then(|res| res.data)
                .map(|mosques| match mosques {
                    MixedMosqueResponse::SingleMosque(mosque) => vec![mosque],
                    MixedMosqueResponse::MosquesVec(mosques) => mosques,
                }),
            None => get_favorite_mosque(None, None)
                .await
                .ok()
                .and_then(|res| res.data)
                .map(|fav| match fav {
                    MixedMosqueResponse::MosquesVec(mosques) => mosques,
                    MixedMosqueResponse::SingleMosque(mosque) => vec![mosque],
                }),
        }
    });

    let visible = Memo::new(move |_| {
        let now = Local::now();
        let is_friday = now.weekday() == Weekday::Fri;
        let all = mosques
            .get()
            .flatten()
            .unwrap_or_default();

        let limited: Vec<NearbyMosqueCardItem> = all
            .into_iter()
            .take(if show_all.get() {
                usize::MAX
            } else {
                PREVIEW_SIZE
            })
            .map(|mosque| {
                let mosque_name = mosque
                    .name
                    .clone()
                    .unwrap_or_else(|| "Nearby mosque".to_string());

                let (iqamah_label, iqamah_time) = mosque
                    .jamat_times
                    .as_ref()
                    .map(|jamat| {
                        let info = next_jamat(jamat, now.time(), is_friday);
                        (
                            format!("{} Iqamah", info.name),
                            format_iqamah(info.time),
                        )
                    })
                    .unwrap_or_else(|| ("Iqamah".to_string(), "--".to_string()));

                NearbyMosqueCardItem::new(
                    mosque.id.clone(),
                    mosque_name,
                    iqamah_label,
                    iqamah_time,
                    0.0,
                    false,
                    mosque.cover_img.clone(),
                )
            })
            .collect();

        limited
    });

    view! {
        <div class="flex gap-5 overflow-x-scroll pb-4">
            <For
                each=move || visible.get()
                key=|item| item.id.clone()
                let(item)
            >
                <NearbyMosqueCard
                    mosque_name=item.mosque_name.clone()
                    iqamah_label=item.iqamah_label.clone()
                    iqamah_time=item.iqamah_time.clone()
                    distance=item.distance
                    is_favorite=item.is_favorite
                    image_url=item
                        .image_url
                        .clone()
                        .unwrap_or_else(|| {
                            "https://images.unsplash.com/photo-1564769662533-4f00a87b4056?auto=format&fit=crop&w=640&h=360&q=80"
                                .to_string()
                        })
                />
            </For>
        </div>
    }
}
