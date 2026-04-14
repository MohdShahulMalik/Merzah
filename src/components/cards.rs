use leptos::prelude::*;

#[component]
pub fn PrayerCard(
    prayer_name: String,
    jamat_time: String,
    adhan_time: String,
    is_current: ReadSignal<bool>
) -> impl IntoView {
    view! {
        <div class:border-green-500=is_current class = "group relative w-[10.25rem] overflow-hidden rounded-2xl border border-indigo-100 bg-white px-3 py-3.5 shadow-[0_0.5rem_1.5rem_rgba(15,23,42,0.08)] transition-all duration-200 hover:-translate-y-0.5 hover:shadow-[0_0.5rem_1.75rem_rgba(15,23,42,0.11)]">
            <p class = "text-[1.05rem] font-semibold tracking-tight text-foreground-900 mb-3 text-center">{prayer_name}</p>

            <div class = "grid grid-cols-[1fr_auto_1fr] items-center gap-2">
                <div class = "text-center">
                    <span class = "block text-[1.25rem] font-semibold leading-none text-slate-900">
                        {jamat_time}
                    </span>
                    <span class = "mt-1.5 block text-[0.65rem] font-medium uppercase tracking-[0.16em] text-foreground-600">
                        "Iqamah"
                    </span>
                </div>

                <div class = "h-11 w-px rounded-full bg-gradient-to-b from-transparent via-slate-300 to-transparent"></div>

                <div class = "text-center">
                    <span class = "block text-[1.25rem] font-semibold leading-none text-slate-900">
                        {adhan_time}
                    </span>
                    <span class = "mt-1.5 block text-[0.65rem] font-medium uppercase tracking-[0.16em] text-foreground-600">
                        "Adhan"
                    </span>
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
                <h2>{highlighted_text(event_type)}</h2>
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
