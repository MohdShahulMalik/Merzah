use leptos::IntoView;
use crate::components::cards::{NextPrayerReminderCard, PrayerCard};
use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    let (is_current, _) = signal(true);
    let (is_not_current, _) = signal(false);

    view! {
        <div class="space-y-8">
            <NextPrayerReminderCard
                location="Brooklyn, NY".to_string()
                mosque_name="Masjid Al-Farooq".to_string()
                prayer_name="Maghrib".to_string()
                iqamah_time="7:48 PM".to_string()
                hours_remaining="02".to_string()
                minutes_remaining="29".to_string()
                seconds_remaining="23".to_string()
            />

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
                    is_current=is_current
                />
                <PrayerCard
                    prayer_name="Maghrib".to_string()
                    jamat_time="7:42 PM".to_string()
                    adhan_time="7:35 PM".to_string()
                    is_current=is_not_current
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
        </div>
    }
}
