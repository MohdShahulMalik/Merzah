use crate::components::cards::{
    EducationalResourceCard, MosqueEventCard, NearbyMosqueCard, NextPrayerReminderCard, PrayerCard,
};
use crate::models::api_responses::MixedMosqueResponse;
use crate::server_functions::mosque::{fetch_mosques_for_location, get_favorite_mosque};
use crate::utils::prayer::next_prayer_card_data;
use crate::utils::prayer::pad2;
use chrono::Datelike;
use chrono::Local;
use chrono::Weekday;
use leptos::IntoView;
use leptos::prelude::*;
use leptos_router::components::A;
use leptos_use::UseGeolocationReturn;
use leptos_use::use_geolocation;
use leptos_use::use_interval_fn;

#[component]
pub fn Home() -> impl IntoView {
    let (is_current, _) = signal(true);
    let (is_not_current, _) = signal(false);

    let UseGeolocationReturn { coords, error: geo_error, .. } = use_geolocation();
    let user_pos = move || coords.get().map(|c| (c.latitude(), c.longitude()));

    let mosque = Resource::new(user_pos, move |pos: Option<(f64, f64)>| async move {
        match pos {
            Some((lat, lon)) => {
                let fav = get_favorite_mosque(Some(lat), Some(lon))
                    .await
                    .ok()
                    .and_then(|res| res.data);
                match fav {
                    Some(MixedMosqueResponse::SingleMosque(mosque)) => Some(mosque),
                    _ => {
                        fetch_mosques_for_location(lat, lon, Some(true))
                            .await
                            .ok()
                            .and_then(|res| res.data)
                            .and_then(|mosques| match mosques {
                                MixedMosqueResponse::SingleMosque(mosque) => Some(mosque),
                                MixedMosqueResponse::MosquesVec(mosques) => mosques.into_iter().next(),
                            })
                    }
                }
            }
            None => {
                get_favorite_mosque(None, None)
                    .await
                    .ok()
                    .and_then(|res| res.data)
                    .and_then(|fav| match fav {
                        MixedMosqueResponse::MosquesVec(mosques) => mosques.into_iter().next(),
                        MixedMosqueResponse::SingleMosque(mosque) => Some(mosque),
                    })
            }
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
        <div class="space-y-8 mt-4 mr-4 mb-4">
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

            <div class="flex flex-wrap gap-4">
                <PrayerCard
                    prayer_name="Fajr".to_string()
                    jamat_time="5:30 AM".to_string()
                    adhan_time="5:00 AM".to_string()
                    is_current=is_not_current
                />
                <PrayerCard
                    prayer_name="Dhuhr".to_string()
                    jamat_time="1:30 PM".to_string()
                    adhan_time="1:00 PM".to_string()
                    is_current=is_not_current
                />
                <PrayerCard
                    prayer_name="Asr".to_string()
                    jamat_time="5:15 PM".to_string()
                    adhan_time="4:45 PM".to_string()
                    is_current=is_not_current
                />
                <PrayerCard
                    prayer_name="Maghrib".to_string()
                    jamat_time="7:42 PM".to_string()
                    adhan_time="7:35 PM".to_string()
                    is_current=is_current
                />
                <PrayerCard
                    prayer_name="Isha".to_string()
                    jamat_time="9:15 PM".to_string()
                    adhan_time="8:45 PM".to_string()
                    is_current=is_not_current
                />
                <PrayerCard
                    prayer_name="Jumu'ah".to_string()
                    jamat_time="1:30 PM".to_string()
                    adhan_time="1:00 PM".to_string()
                    is_current=is_not_current
                />
            </div>

            <section class="space-y-5">
                <div class="flex items-center justify-between">
                    <h2 class="text-2xl font-bold text-purple-900">"Nearby Mosques"</h2>
                    <A href="#" attr:class="font-medium text-purple-600 hover:text-purple-700">"View All →"</A>
                </div>
                <div class="flex gap-5 overflow-x-scroll pb-4">
                    <NearbyMosqueCard
                        mosque_name="Masjid Al-Farooq".to_string()
                        iqamah_label="Maghrib Iqamah".to_string()
                        iqamah_time="7:48 PM".to_string()
                        distance=0.2
                        is_favorite=true
                        image_url="https://images.unsplash.com/photo-1584551246679-0daf3d275d0f?auto=format&fit=crop&w=640&h=360&q=80".to_string()
                    />
                    <NearbyMosqueCard
                        mosque_name="Islamic Center of Brooklyn".to_string()
                        iqamah_label="Maghrib Iqamah".to_string()
                        iqamah_time="7:50 PM".to_string()
                        distance=0.8
                        is_favorite=false
                        image_url="https://images.unsplash.com/photo-1519817650390-64a93db51149?auto=format&fit=crop&w=640&h=360&q=80".to_string()
                    />
                    <NearbyMosqueCard
                        mosque_name="Masjid At-Taqwa".to_string()
                        iqamah_label="Maghrib Iqamah".to_string()
                        iqamah_time="7:45 PM".to_string()
                        distance=1.2
                        is_favorite=false
                        image_url="https://images.unsplash.com/photo-1512632578888-169bbbc64f33?auto=format&fit=crop&w=640&h=360&q=80".to_string()
                    />
                    <NearbyMosqueCard
                        mosque_name="Muslim Community Center".to_string()
                        iqamah_label="Maghrib Iqamah".to_string()
                        iqamah_time="7:52 PM".to_string()
                        distance=2.1
                        is_favorite=false
                        image_url="https://images.unsplash.com/photo-1548013146-72479768bada?auto=format&fit=crop&w=640&h=360&q=80".to_string()
                    />
                </div>
            </section>

            <section class="space-y-5">
                <div class="flex items-center justify-between">
                    <h2 class="text-2xl font-bold text-purple-900">"Upcoming Events"</h2>
                    <A href="#" attr:class="font-medium text-purple-600 hover:text-purple-700">"View All →"</A>
                </div>
                <div class="flex gap-5 overflow-x-scroll pb-4">
                    <MosqueEventCard
                        event_title="Jummah Prayer & Khutbah".to_string()
                        event_type="Khutbah".to_string()
                        event_type_class="bg-[#d9e0ff] text-[#18206d]".to_string()
                        mosque_name="Masjid Al-Farooq".to_string()
                        event_day="Friday".to_string()
                        event_time="1:15 PM".to_string()
                        event_short_description="Weekly Friday sermon focusing on building community ties and strengthening our faith.".to_string()
                    />
                    <MosqueEventCard
                        event_title="Understanding the Quran: Surah Al-Kahf".to_string()
                        event_type="Lecture".to_string()
                        event_type_class="bg-[#7debf0] text-[#064f68]".to_string()
                        mosque_name="Islamic Center of Brooklyn".to_string()
                        event_day="Saturday".to_string()
                        event_time="7:00 PM".to_string()
                        event_short_description="Deep dive into the meanings and lessons from Surah Al-Kahf with Sheikh Abdullah.".to_string()
                    />
                    <MosqueEventCard
                        event_title="Community Breakfast & Youth Program".to_string()
                        event_type="Community".to_string()
                        event_type_class="bg-[#b9f4bd] text-[#11631c]".to_string()
                        mosque_name="Masjid At-Taqwa".to_string()
                        event_day="Sunday".to_string()
                        event_time="11:00 AM".to_string()
                        event_short_description="Join us for a family-friendly breakfast and engaging activities for children and youth.".to_string()
                    />
                </div>
            </section>

            <section class="space-y-5">
                <div class="flex items-center justify-between">
                    <h2 class="text-2xl font-bold text-purple-900">"Recommended"</h2>
                    <A href="#" attr:class="font-medium text-purple-600 hover:text-purple-700">"View All →"</A>
                </div>
                <div class="flex gap-5 overflow-x-scroll pb-4">
                    <EducationalResourceCard
                        lesson_count="12 lessons".to_string()
                        level="Beginner".to_string()
                        resource_title="Introduction to Tajweed".to_string()
                        resource_by="Sheikh Muhammad Ali".to_string()
                        action_label="Continue Learning".to_string()
                        image_url="https://images.unsplash.com/photo-1609599006353-e629aaabfeae?auto=format&fit=crop&w=640&h=360&q=80".to_string()
                    />
                    <EducationalResourceCard
                        lesson_count="8 lessons".to_string()
                        level="All Levels".to_string()
                        resource_title="Ramadan Preparation Guide".to_string()
                        resource_by="Ustadha Fatima Hassan".to_string()
                        action_label="Start Course".to_string()
                        image_url="https://images.unsplash.com/photo-1542816417-0983c9c9ad53?auto=format&fit=crop&w=640&h=360&q=80".to_string()
                    />
                    <EducationalResourceCard
                        lesson_count="15 lessons".to_string()
                        level="Intermediate".to_string()
                        resource_title="The Art of Du'a".to_string()
                        resource_by="Imam Yusuf Rahman".to_string()
                        action_label="Continue Learning".to_string()
                        image_url="https://images.unsplash.com/photo-1591604129939-f1efa4d9f7fa?auto=format&fit=crop&w=640&h=360&q=80".to_string()
                    />
                    <EducationalResourceCard
                        lesson_count="20 lessons".to_string()
                        level="Advanced".to_string()
                        resource_title="Seerah: Life of the Prophet ﷺ".to_string()
                        resource_by="Dr. Omar Suleiman".to_string()
                        action_label="Start Course".to_string()
                        image_url="https://images.unsplash.com/photo-1481627834876-b7833e8f5570?auto=format&fit=crop&w=640&h=360&q=80".to_string()
                    />
                    <EducationalResourceCard
                        lesson_count="10 lessons".to_string()
                        level="Beginner".to_string()
                        resource_title="Islamic Finance Basics".to_string()
                        resource_by="Sheikh Ahmad Bilal".to_string()
                        action_label="Start Course".to_string()
                    />
                </div>
            </section>
        </div>
    }
}
