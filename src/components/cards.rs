use leptos::prelude::*;

#[component]
pub fn PrayerCard(
    prayer_name: String,
    jamat_time: String,
    adhan_time: String,
) -> impl IntoView {
    view! {
        <div class = "bg-white">
            <p>{prayer_name}</p>
            <div class = "flex justify-between">
                <div class = "grid">
                    <span>{jamat_time}</span>
                    <span>Iqamah</span>
                </div>
                <div class = "grid">
                    <span>{adhan_time}</span>
                    <span>Adhan</span>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn nearby_mosques_card(
    mosque_name: String,
    next_prayer: String,
    jamat_time: String,
    distance: f64,
) -> impl IntoView {
    view! {
        <div>
            <div></div>
            <div>
                <h1>{mosque_name}</h1>
                <div class = "grid">
                    <span>{distance} " • Next: "{next_prayer}</span>
                    <span>"Jamat Time: "{jamat_time}</span>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn mosque_events_card(
    event_title: String,
    event_type: String,
    mosque_name: String,
    event_time: String,
    event_short_description: String,
) -> impl IntoView {
    view! {
        <div>
            <div class = "flex justify-between">
                <h1>{event_title}</h1>
                <h2>{event_type}</h2>
            </div>
            <div class = "grid">
                <span>{mosque_name}" • "{event_time}</span>
                <span>{event_short_description}</span>
            </div>
        </div>
    }
}

#[component]
pub fn educational_resources_card(
    resource_title: String,
    resource_short_description: String,
    resource_by: String,
) -> impl IntoView {
    view! {
        <div>
            <div></div>
            <div class = "grid">
                <h1>{resource_title}</h1>
                <h2>{resource_short_description}</h2>
                <span>{resource_by}</span>
            </div>
        </div>
    }
}
