use leptos::prelude::*;

#[component]
pub fn PrayerCard(
    prayer_name: String,
    jamat_time: String,
    adhan_time: String,
    is_current: ReadSignal<bool>
) -> impl IntoView {
    let border_classes = move || {
        if is_current.get() {
            "ring-3 ring-green-500"
        } else {
            ""
        }
    };

    view! {
        <div class=move || format!("group relative w-[10.25rem] overflow-hidden rounded-2xl bg-surface-700 border-t-3 border-t-white px-3 py-3.5 transition-all duration-200 hover:-translate-y-0.5 hover:shadow-[0_0.5rem_1.75rem_rgba(15,23,42,0.18)] hover:scale-[1.03] {}", border_classes())>
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
            <div class="pointer-events-none absolute inset-0">
                <svg class="absolute top-[12%] left-[8%]" width="3" height="3"><circle cx="1.5" cy="1.5" r="1.5" fill="white" opacity="0.8"/></svg>
                <svg class="absolute top-[20%] left-[20%]" width="4" height="4"><circle cx="2" cy="2" r="2" fill="white" opacity="0.6"/></svg>
                <svg class="absolute top-[8%] left-[35%]" width="3" height="3"><circle cx="1.5" cy="1.5" r="1.5" fill="white" opacity="0.7"/></svg>
                <svg class="absolute top-[25%] left-[50%]" width="1" height="1"><circle cx="0.5" cy="0.5" r="0.5" fill="white" opacity="0.9"/></svg>
                <svg class="absolute top-[15%] left-[65%]" width="3" height="3"><circle cx="1.5" cy="1.5" r="1.5" fill="white" opacity="0.5"/></svg>
                <svg class="absolute top-[5%] left-[80%]" width="4" height="4"><circle cx="2" cy="2" r="2" fill="white" opacity="0.6"/></svg>
                <svg class="absolute top-[30%] left-[12%]" width="1" height="1"><circle cx="0.5" cy="0.5" r="0.5" fill="white" opacity="0.7"/></svg>
                <svg class="absolute top-[18%] left-[90%]" width="3" height="3"><circle cx="1.5" cy="1.5" r="1.5" fill="white" opacity="0.8"/></svg>
                <svg class="absolute top-[35%] left-[75%]" width="3" height="3"><circle cx="1.5" cy="1.5" r="1.5" fill="white" opacity="0.5"/></svg>
                <svg class="absolute top-[40%] left-[45%]" width="1" height="1"><circle cx="0.5" cy="0.5" r="0.5" fill="white" opacity="0.6"/></svg>
                <svg class="absolute top-[8%] left-[55%]" width="3" height="3"><circle cx="1.5" cy="1.5" r="1.5" fill="white" opacity="0.7"/></svg>
                <svg class="absolute top-[22%] left-[95%]" width="1" height="1"><circle cx="0.5" cy="0.5" r="0.5" fill="white" opacity="0.5"/></svg>
            </div>
            <div class="pointer-events-none absolute inset-x-0 bottom-0 h-24 bg-[linear-gradient(180deg,transparent_0%,rgba(162,132,230,0.12)_100%)]"></div>
            <div class="pointer-events-none absolute inset-x-0 bottom-0 h-12 mix-blend-soft-light opacity-30">
                <svg viewBox="0 0 400 50" preserveAspectRatio="xMidYMax slice" class="w-full h-full fill-white">
                    <path d="M0,50 L0,40 L20,40 L20,32 L25,28 L30,32 L30,40 L50,40 L50,50 Z M70,50 L70,35 L80,35 L80,22 L90,12 L100,22 L100,35 L110,35 L110,50 Z M130,50 L130,42 L140,42 L140,38 L145,34 L150,38 L150,42 L160,42 L160,50 Z M180,50 L180,30 L190,30 L190,18 Q205,2 220,18 L220,30 L230,30 L230,50 Z M200,50 L200,38 Q200,30 210,30 Q220,30 220,38 L220,50 Z M250,50 L250,42 L260,42 L260,36 L265,32 L270,36 L270,42 L280,42 L280,50 Z M300,50 L300,35 L310,35 L310,26 L320,18 L330,26 L330,35 L340,35 L340,50 Z M360,50 L360,44 L370,44 L370,48 L380,48 L380,50 Z M385,50 L385,46 L395,46 L395,50 Z" />
                </svg>
            </div>
            <div class="pointer-events-none absolute -left-12 top-10 h-36 w-36 rounded-full bg-amber-300/10 blur-3xl"></div>
            <div class="pointer-events-none absolute right-0 top-0 h-full w-full bg-[radial-gradient(circle_at_85%_25%,rgba(255,219,137,0.12),transparent_18%),radial-gradient(circle_at_18%_18%,rgba(255,255,255,0.12),transparent_14%)]"></div>

            <div class="relative flex gap-8 lg:flex-row lg:items-center lg:justify-between">
                <div class="flex max-w-2xl justify-center items-center">
                    <div class="mb-6 flex items-center gap-3 text-violet-100/85">
                    </div>

                    <div class="space-y-3">
                        <div class="inline-flex items-center rounded-full border border-white/12 bg-white/10 px-4 py-2 text-sm font-medium text-violet-50 shadow-[0_0.5rem_1rem_rgba(11,6,27,0.18)]">
                            {mosque_name}
                        </div>
                        <h2 class="text-[1.75rem] font-bold tracking-tight text-white md:text-[2.5rem] lg:text-[3rem] [text-shadow:0_0_30px_rgba(200,160,20,0.5),0_0_60px_rgba(200,160,20,0.25)]">
                            {prayer_name}
                        </h2>
                        <p class="text-base text-violet-100/78 md:text-lg flex gap-1 items-center">
                            <span class="text-[0.75rem] text-violet-100/55">"IQAMAH"</span>
                            <span class="font-semibold text-amber-200">{iqamah_time}</span>
                        </p>
                    </div>
                </div>

                <div class="absolute bottom-4 left-1/2 -translate-x-1/2 flex items-center gap-1.5 text-[0.68rem] text-violet-100/60">
                    <span>"📍"</span>
                    <span>{location}</span>
                </div>

                <div class="grid grid-cols-3 gap-3 sm:gap-4">
                    {vec![
                        (hours_remaining, "Hrs"),
                        (minutes_remaining, "Min"),
                        (seconds_remaining, "Sec"),
                    ]
                    .into_iter()
                    .map(|(value, label)| {
                        view! {
                            <div class="grid justify-items-center">
                                <div class="rounded-[1.5rem] border border-white/10 bg-slate-950/45 px-4 py-4 text-center shadow-[0_0.75rem_1.6rem_rgba(8,5,20,0.24)] backdrop-blur">
                                    <p class="text-4xl font-bold leading-none text-amber-100">{value}</p>
                                </div>
                                <p class="mt-1 text-[0.6rem] font-bold uppercase tracking-[0.26em] text-violet-100/55">
                                    {label}
                                </p>
                            </div>
                        }
                    })
                    .collect::<Vec<_>>()}
                </div>
            </div>
        </section>
    }
}

#[component]
pub fn NearbyMosqueCard(
    mosque_name: String,
    iqamah_label: String,
    iqamah_time: String,
    distance: f64,
    is_favorite: bool,
) -> impl IntoView {
    view! {
        <article class="rounded-2xl bg-white ring-1 ring-indigo-950/5 w-[25%]">
            <div class="relative flex h-[7.75rem] items-center justify-center bg-[#c9cef3]">
                <span class="text-4xl">"🕌"</span>
                <div class="absolute right-4 top-2 flex h-14 w-14 items-center justify-center rounded-full bg-white/90 text-lg shadow-sm cursor-pointer transition-all">
                    {if is_favorite { "❤️" } else { "🤍" }}
                </div>
            </div>

            <div class="space-y-3 p-4">
                <div>
                    <h3 class="text-[0.9rem] font-bold leading-tight text-[#050047]">{mosque_name}</h3>
                    <p class="mt-1 text-sm text-[#17135f]">
                        <span class="mr-2 text-pink-500">"⚲"</span>
                        {format!("{distance:.1} km away")}
                    </p>
                </div>

                <div class="flex items-center justify-between rounded-lg bg-[#e8edff] px-3 py-2 text-sm text-[#211c74]">
                    <span>{iqamah_label}</span>
                    <span class="font-bold">{iqamah_time}</span>
                </div>

            </div>
        </article>
    }
}

#[component]
pub fn MosqueEventCard(
    event_title: String,
    event_type: String,
    event_type_class: String,
    mosque_name: String,
    event_day: String,
    event_time: String,
    event_short_description: String,
) -> impl IntoView {
    view! {
        <article class="shrink-0 w-[32.2%] rounded-xl bg-white p-4 ring-1 ring-indigo-950/10">
            <div class="mb-3 flex items-start justify-between gap-3">
                <span class=format!("rounded-md px-2 py-0.5 text-[0.75rem] font-bold uppercase tracking-wider {}", event_type_class)>
                    {event_type}
                </span>
                <div class="text-right text-[#050047]">
                    <p class="text-[0.85rem] font-bold">{event_day}</p>
                    <p class="mt-0.5 text-xs">{event_time}</p>
                </div>
            </div>
            <div class="space-y-1.5">
                <h3 class="text-[0.9rem] font-bold leading-tight text-[#050047]">{event_title}</h3>
                <p class="text-xs text-foreground-600">"🕌 "{mosque_name}</p>
                <p class="text-[0.8rem] leading-relaxed text-[#302977]">{event_short_description}</p>
            </div>
        </article>
    }
}

#[component]
pub fn EducationalResourceCard(
    icon: String,
    lesson_count: String,
    level: String,
    resource_title: String,
    resource_by: String,
    action_label: String,
) -> impl IntoView {
    view! {
        <article class="shrink-0 overflow-hidden rounded-xl bg-white ring-1 ring-indigo-950/5 w-[25%]">
            <div class="relative flex h-[8.5rem] items-center justify-center bg-[#211f55]">
                <span class="text-4xl">{icon}</span>
                <div class="absolute inset-x-0 bottom-0 h-1 bg-[#d8d7e7]">
                    <div class="h-full w-[65%] bg-[#f0bd25]"></div>
                </div>
            </div>

            <div class="space-y-3 px-3 py-3">
                <div class="flex flex-wrap gap-2">
                    <span class="rounded-full bg-[#e8edff] px-2 py-1 text-[0.7rem] font-medium text-[#211c74]">
                        {lesson_count}
                    </span>
                    <span class="rounded-full bg-[#e8edff] px-2 py-1 text-[0.7rem] font-medium text-[#211c74]">
                        {level}
                    </span>
                </div>

                <div>
                    <h3 class="text-base font-bold leading-tight text-[#050047]">{resource_title}</h3>
                    <p class="mt-1 text-sm text-[#17135f]">{resource_by}</p>
                </div>

                <button class="w-full rounded-lg bg-[#e8edff] px-3 py-2.5 text-sm font-medium text-[#050047] transition-colors hover:bg-[#dbe3ff]">
                    {action_label}
                </button>
            </div>
        </article>
    }
}
