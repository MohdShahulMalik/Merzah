use leptos::IntoView;
use crate::components::cards::PrayerCard;
use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div>
            <PrayerCard
                prayer_name="Fajr".to_string()
                jamat_time="5:30 AM".to_string()
                adhan_time="5:00 AM".to_string()
            />
        </div>
    }
}
