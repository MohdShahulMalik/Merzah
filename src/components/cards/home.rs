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
        <div class=move || format!("group relative w-[10.25rem] overflow-hidden rounded-2xl bg-white border-t-white px-3 py-3.5 transition-all duration-200 hover:-translate-y-0.5 hover:shadow-[0_0.5rem_1.75rem_rgba(15,23,42,0.18)] hover:scale-[1.03] {}", border_classes())>
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
        <section class="relative overflow-hidden rounded-4xl border border-violet-900/20 bg-[radial-gradient(circle_at_top,_rgba(255,255,255,0.16),_transparent_28%),linear-gradient(135deg,_#23104a_0%,_#31105d_48%,_#24103f_100%)] px-6 py-7 text-white shadow-[0_1.5rem_4rem_rgba(37,16,79,0.22)]">
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

            <div class="relative flex gap-8 flex-row items-center justify-between">
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

                <div class="absolute bottom-4 left-1/2 -translate-x-1/2 flex items-center gap-1.5 text-[0.68rem] text-violet-100/60 max-[840px]:bottom-1">
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
    #[prop(optional)] image_url: Option<String>,
) -> impl IntoView {
    let image_url = image_url.unwrap_or_else(|| {
        "https://images.unsplash.com/photo-1564769662533-4f00a87b4056?auto=format&fit=crop&w=640&h=360&q=80".to_string()
    });
    let image_alt = format!("{} mosque", mosque_name);

    view! {
        <article class="group relative w-[18rem] overflow-hidden rounded-2xl bg-white ring-1 ring-indigo-950/5 transition-[background-color,ring-color] duration-500 ease-out hover:bg-[#fbf8ef] hover:ring-[#d8c07a]/70">
            <div class="relative h-31 overflow-hidden bg-[#c9cef3]">
                <img
                    src=image_url
                    alt=image_alt
                    class="h-full w-full object-cover [object-position:center_48%] transition-[object-position,filter] duration-700 ease-out group-hover:[object-position:center_56%] group-hover:[filter:sepia(0.08)_saturate(0.98)_contrast(1.02)]"
                />
                <div class="pointer-events-none absolute inset-0 bg-linear-to-t from-[#050047]/60 via-[#050047]/12 to-white/0"></div>

                <div class="absolute right-3 top-3 flex h-9 w-9 cursor-pointer items-center justify-center rounded-full border border-white/25 bg-white/18 text-white backdrop-blur-md transition-[background-color,border-color,color] duration-300 ease-out group-hover:border-white/55 group-hover:bg-white group-hover:text-pink-600">
                    <svg
                        class=if is_favorite { "h-4 w-4 fill-pink-500 stroke-pink-500" } else { "h-4 w-4 fill-none stroke-current" }
                        viewBox="0 0 24 24"
                        stroke-width="1.8"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        aria-hidden="true"
                    >
                        <path d="M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78L12 21.23l8.84-8.84a5..5 5.5 0 0 0 0-7.78Z" />
                    </svg>
                </div>

                <div class="absolute inset-x-0 bottom-0 translate-y-full border-t border-white/15 bg-[#211f55]/88 px-4 py-3 text-white backdrop-blur-md transition-transform duration-500 ease-out group-hover:translate-y-0">
                    <div class="flex items-center justify-start">
                        <span class="text-sm font-bold text-amber-200">"View details →"</span>
                    </div>
                </div>
            </div>

            <div class="relative space-y-3 p-4">
                <div class="pointer-events-none absolute inset-x-4 top-0 h-px bg-linear-to-r from-transparent via-[#f0bd25]/0 to-transparent transition-colors duration-500 ease-out group-hover:via-[#f0bd25]/70"></div>

                <div>
                    <h3 class="text-[0.9rem] font-bold leading-tight text-[#050047] transition-colors duration-300 ease-out group-hover:text-[#211f55]">
                        {mosque_name}
                    </h3>
                    <p class="mt-2 flex items-center gap-1.5 text-sm text-[#17135f] transition-colors duration-300 ease-out group-hover:text-[#4a3d86]">
                        <svg
                            class="h-3.5 w-3.5 text-[#f0bd25]"
                            viewBox="0 0 24 24"
                            fill="none"
                            stroke="currentColor"
                            stroke-width="2"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            aria-hidden="true"
                        >
                            <path d="M20 10c0 4.993-5.539 10.193-7.399 11.799a1 1 0 0 1-1.202 0C9.539 20.193 4 14.993 4 10a8 8 0 0 1 16 0Z" />
                            <circle cx="12" cy="10" r="3" />
                        </svg>
                        <span>{format!("{distance:.1} km away")}</span>
                    </p>
                </div>

                <div class="relative flex items-center justify-between rounded-lg border border-transparent bg-[#e8edff] px-3 py-2 text-sm text-[#211c74] transition-[background-color,border-color] duration-300 ease-out group-hover:border-[#ead58d] group-hover:bg-white/70">
                    <span class="text-[0.72rem] font-medium uppercase tracking-[0.08em] text-[#5c5793] transition-colors duration-300 ease-out group-hover:text-[#7c6a2e]">{iqamah_label}</span>
                    <span class="font-bold text-[#211c74] transition-colors duration-300 ease-out group-hover:text-[#6b4e00]">{iqamah_time}</span>
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
        <article class="group relative w-98 shrink-0 overflow-hidden rounded-xl bg-white p-4 ring-1 ring-indigo-950/10 transition-[background-color,ring-color] duration-300 ease-out hover:ring-purple-300/70">

            <div class="relative mb-3 flex items-start justify-between gap-3">
                <span class=format!("rounded-md px-2 py-0.5 text-[0.75rem] font-bold uppercase tracking-wider transition-[transform,letter-spacing] duration-300 ease-out group-hover:tracking-[0.14em] {}", event_type_class)>
                    {event_type}
                </span>
                <div class="rounded-lg px-2 py-1 text-right text-[#050047] transition-colors duration-300 ease-out group-hover:bg-indigo-50">
                    <p class="text-[0.85rem] font-bold">{event_day}</p>
                    <p class="mt-0.5 text-xs">{event_time}</p>
                </div>
            </div>
            <div class="relative space-y-1.5">
                <h3 class="text-[0.9rem] font-bold leading-tight text-[#050047] transition-colors duration-300 ease-out group-hover:text-purple-900">{event_title}</h3>
                <p class="text-xs text-foreground-600 transition-colors duration-300 ease-out group-hover:text-indigo-700">"🕌 "{mosque_name}</p>
                <p class="text-[0.8rem] leading-relaxed text-[#302977]">{event_short_description}</p>
            </div>
        </article>
    }
}

#[component]
pub fn EducationalResourceCard(
    lesson_count: String,
    level: String,
    resource_title: String,
    resource_by: String,
    action_label: String,
    #[prop(optional)] image_url: Option<String>,
) -> impl IntoView {
    let image_url = image_url.unwrap_or_else(|| {
        "https://images.unsplash.com/photo-1542816417-0983c9c9ad53?auto=format&fit=crop&w=640&h=360&q=80".to_string()
    });
    let image_alt = format!("{} resource cover", resource_title);

    view! {
        <article class="group relative w-82 shrink-0 overflow-hidden rounded-xl bg-white ring-1 ring-indigo-950/5 transition-[background-color,ring-color] duration-300 ease-out hover:ring-purple-200">
            <div class="relative h-34 overflow-hidden bg-[#211f55]">
                <img
                    src=image_url
                    alt=image_alt
                    class="h-full w-full object-cover transition-[filter] duration-500 ease-out group-hover:[filter:brightness(0.76)_saturate(0.9)]"
                />
                <div class="pointer-events-none absolute inset-0 bg-linear-to-t from-[#211f55]/72 via-[#211f55]/18 to-transparent"></div>

                <div class="pointer-events-none absolute left-3 top-3 h-8 w-8 -translate-x-1 -translate-y-1 border-l border-t border-white/0 opacity-0 transition-[border-color,opacity,transform] duration-500 ease-out group-hover:translate-x-0 group-hover:translate-y-0 group-hover:border-white/70 group-hover:opacity-100"></div>
                <div class="pointer-events-none absolute bottom-3 right-3 h-8 w-8 translate-x-1 translate-y-1 border-b border-r border-white/0 opacity-0 transition-[border-color,opacity,transform] duration-500 ease-out group-hover:translate-x-0 group-hover:translate-y-0 group-hover:border-white/70 group-hover:opacity-100"></div>

            </div>

            <div class="relative space-y-3 px-3 py-3">
                <div class="pointer-events-none absolute inset-y-4 left-0 w-px bg-linear-to-b from-transparent via-amber-300 to-transparent opacity-0 transition-opacity duration-500 ease-out group-hover:opacity-100"></div>

                <div class="flex flex-wrap gap-2">
                    <span class="rounded-full bg-[#e8edff] px-2 py-1 text-[0.7rem] font-medium text-[#211c74] transition-colors duration-300 ease-out group-hover:text-[#4c1d95]">
                        {lesson_count}
                    </span>
                    <span class="rounded-full bg-[#e8edff] px-2 py-1 text-[0.7rem] font-medium text-[#211c74] transition-colors duration-300 ease-out group-hover:text-[#4c1d95]">
                        {level}
                    </span>
                </div>

                <div>
                    <h3 class="relative inline-block text-base font-bold leading-tight text-[#050047] transition-colors duration-300 ease-out group-hover:text-purple-900">
                        <span>{resource_title}</span>
                        <span class="absolute -bottom-1 left-0 h-px w-full origin-left scale-x-0 bg-amber-300 transition-transform duration-500 ease-out group-hover:scale-x-100"></span>
                    </h3>
                    <p class="mt-2 text-sm text-[#17135f] transition-colors duration-300 ease-out group-hover:text-indigo-700">{resource_by}</p>
                </div>

                <button class="flex w-full items-center justify-center gap-2 rounded-lg bg-purple-600 px-3 py-2.5 text-sm font-medium text-white transition-colors duration-300 ease-out hover:bg-purple-700">
                    <span>{action_label}</span>
                    <span class="transition-transform duration-300 ease-out group-hover:translate-x-1" aria-hidden="true">"→"</span>
                </button>
            </div>
        </article>
    }
}
