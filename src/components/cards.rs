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
pub fn NextPrayerReminderCard(
    location: String,
    mosque_name: String,
    prayer_name: String,
    iqamah_time: String,
    hours_remaining: String,
    minutes_remaining: String,
    seconds_remaining: String,
) -> impl IntoView {
    view! {
        <section class="relative overflow-hidden rounded-[2rem] border border-violet-900/20 bg-[radial-gradient(circle_at_top,_rgba(255,255,255,0.16),_transparent_28%),linear-gradient(135deg,_#23104a_0%,_#31105d_48%,_#24103f_100%)] px-6 py-7 text-white shadow-[0_1.5rem_4rem_rgba(37,16,79,0.22)] md:px-10 md:py-9">
            <div class="pointer-events-none absolute inset-x-0 bottom-0 h-24 bg-[linear-gradient(180deg,transparent_0%,rgba(162,132,230,0.12)_100%)]"></div>
            <div class="pointer-events-none absolute -left-12 top-10 h-36 w-36 rounded-full bg-amber-300/10 blur-3xl"></div>
            <div class="pointer-events-none absolute right-0 top-0 h-full w-full bg-[radial-gradient(circle_at_85%_25%,rgba(255,219,137,0.12),transparent_18%),radial-gradient(circle_at_18%_18%,rgba(255,255,255,0.12),transparent_14%)]"></div>

            <div class="relative flex flex-col gap-8 lg:flex-row lg:items-end lg:justify-between">
                <div class="max-w-2xl">
                    <div class="mb-6 flex items-center gap-3 text-violet-100/85">
                        <div class="grid h-12 w-12 place-items-center rounded-2xl border border-amber-200/20 bg-white/10 text-2xl shadow-[0_0.5rem_1.25rem_rgba(10,4,24,0.18)]">
                            "☾"
                        </div>
                        <div class="space-y-1">
                            <p class="text-[0.72rem] font-semibold uppercase tracking-[0.28em] text-violet-100/70">
                                "Next Prayer"
                            </p>
                            <p class="text-sm text-violet-100/80">{location}</p>
                        </div>
                    </div>

                    <div class="space-y-3">
                        <div class="inline-flex items-center rounded-full border border-white/12 bg-white/10 px-4 py-2 text-sm font-medium text-violet-50 shadow-[0_0.5rem_1rem_rgba(11,6,27,0.18)]">
                            {mosque_name}
                        </div>
                        <h2 class="text-5xl font-semibold tracking-tight text-white md:text-6xl">
                            {prayer_name}
                        </h2>
                        <p class="text-base text-violet-100/78 md:text-lg">
                            "Iqamah at "
                            <span class="font-semibold text-amber-200">{iqamah_time}</span>
                        </p>
                    </div>
                </div>

                <div class="grid grid-cols-3 gap-3 self-start sm:gap-4 lg:self-end">
                    <div class="min-w-24 rounded-[1.5rem] border border-white/10 bg-slate-950/35 px-4 py-4 text-center shadow-[0_0.75rem_1.6rem_rgba(8,5,20,0.24)] backdrop-blur">
                        <p class="text-4xl font-semibold leading-none text-amber-100">{hours_remaining}</p>
                        <p class="mt-3 text-[0.68rem] font-semibold uppercase tracking-[0.26em] text-violet-100/55">
                            "Hours"
                        </p>
                    </div>
                    <div class="min-w-24 rounded-[1.5rem] border border-white/10 bg-slate-950/35 px-4 py-4 text-center shadow-[0_0.75rem_1.6rem_rgba(8,5,20,0.24)] backdrop-blur">
                        <p class="text-4xl font-semibold leading-none text-amber-100">{minutes_remaining}</p>
                        <p class="mt-3 text-[0.68rem] font-semibold uppercase tracking-[0.26em] text-violet-100/55">
                            "Minutes"
                        </p>
                    </div>
                    <div class="min-w-24 rounded-[1.5rem] border border-white/10 bg-slate-950/35 px-4 py-4 text-center shadow-[0_0.75rem_1.6rem_rgba(8,5,20,0.24)] backdrop-blur">
                        <p class="text-4xl font-semibold leading-none text-amber-100">{seconds_remaining}</p>
                        <p class="mt-3 text-[0.68rem] font-semibold uppercase tracking-[0.26em] text-violet-100/55">
                            "Seconds"
                        </p>
                    </div>
                </div>
            </div>
        </section>
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
