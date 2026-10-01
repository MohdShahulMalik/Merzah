use crate::app::AppState;
use crate::app::AppStateStoreFields;
use crate::components::cards::NextPrayerReminderCard;
use crate::models::api_responses::MixedMosqueResponse;
use crate::server_functions::mosque::{fetch_mosques_for_location, get_favorite_mosque};
use crate::utils::prayer::next_prayer_card_data;
use crate::utils::prayer::pad2;
use chrono::Datelike;
use chrono::Local;
use chrono::Weekday;
use leptos::IntoView;
use leptos::prelude::*;
use leptos_use::use_interval_fn;
use reactive_stores::OptionStoreExt;
use reactive_stores::Store;

#[component]
pub fn NextPrayerSection() -> impl IntoView {
    let app_state = expect_context::<Store<AppState>>();
    let user_pos = move || app_state.coords().map(|coords| coords.read().as_lat_lon());

    let mosque = Resource::new(user_pos, move |pos: Option<(f64, f64)>| async move {
        match pos {
            Some((lat, lon)) => {
                let fav = get_favorite_mosque(Some(lat), Some(lon))
                    .await
                    .ok()
                    .and_then(|res| res.data);

                match fav {
                    Some(MixedMosqueResponse::SingleMosque(mosque)) => Some(mosque),
                    _ => fetch_mosques_for_location(lat, lon, Some(true))
                        .await
                        .ok()
                        .and_then(|res| res.data)
                        .and_then(|mosques| match mosques {
                            MixedMosqueResponse::SingleMosque(mosque) => Some(mosque),
                            MixedMosqueResponse::MosquesVec(mosques) => mosques.into_iter().next(),
                        }),
                }
            }

            None => get_favorite_mosque(None, None)
                .await
                .ok()
                .and_then(|res| res.data)
                .and_then(|fav| match fav {
                    MixedMosqueResponse::MosquesVec(mosques) => mosques.into_iter().next(),
                    MixedMosqueResponse::SingleMosque(mosque) => Some(mosque),
                }),
        }
    });

    let (recompute, set_recompute) = signal(0_u32);
    let (remaining, set_remaining) = signal(0_u64);

    let next_prayer = Memo::new(move |_| {
        recompute.get();
        let now = Local::now();

        mosque
            .get()
            .flatten()
            .map(|mosque| next_prayer_card_data(&mosque, now.time(), now.weekday() == Weekday::Fri))
    });

    Effect::new(move |_| {
        if let Some(card) = next_prayer.get() {
            set_remaining.set(card.total_seconds);
        }
    });

    use_interval_fn(
        move || {
            if remaining.get() > 0 {
                set_remaining.update(|secs| *secs -= 1);
            } else if next_prayer.get().is_some_and(|card| card.has_times) {
                set_recompute.update(|value| *value += 1);
            }
        },
        1000,
    );

    view! {
        {move || {
            let card = next_prayer.get();
            let location = card
                .as_ref()
                .map(|card| card.location.clone())
                .unwrap_or_else(|| "Locating…".to_string());
            let mosque_name = card
                .as_ref()
                .map(|card| card.mosque_name.clone())
                .unwrap_or_else(|| "Finding your mosque…".to_string());
            let prayer_name = card
                .as_ref()
                .map(|card| card.prayer_name.clone())
                .unwrap_or_else(|| "—".to_string());
            let iqamah_time = card
                .as_ref()
                .map(|card| card.iqamah_time.clone())
                .unwrap_or_else(|| "—".to_string());
            let left = remaining.get();
            let hours = pad2(left / 3600);
            let minutes = pad2((left % 3600) / 60);
            let seconds = pad2(left % 60);

            view! {
                <NextPrayerReminderCard
                    location=location
                    mosque_name=mosque_name
                    prayer_name=prayer_name
                    iqamah_time=iqamah_time
                    hours_remaining=hours
                    minutes_remaining=minutes
                    seconds_remaining=seconds
                />
            }
        }}
    }
}
