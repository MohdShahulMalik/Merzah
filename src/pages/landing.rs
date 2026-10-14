use leptos::prelude::*;

#[component]
pub fn Landing() -> impl IntoView {
    view! {
        <div class="font-sans antialiased overflow-x-hidden bg-surface-800 text-foreground-900">
            // ============================ HERO ============================
            <header class="relative flex flex-col text-center">
                <div class="relative z-0 mx-auto flex w-full max-w-3xl flex-col items-center px-6 pt-14 pb-36 md:pt-20">
                    <a href="#" class="inline-flex flex-col items-center select-none" aria-label="Merzah home">
                        <img
                            src="/assets/logo.png"
                            alt="Merzah emblem"
                            class="w-24 h-24 md:w-28 md:h-28 rounded-full object-cover shadow-lg shadow-purple-950/20 ring-1 ring-purple-950/10"
                        />
                        <img src="/assets/logo-text.png" alt="Merzah" class="mt-6 h-14 md:h-16 w-auto" />
                    </a>
                    <h1 class="mt-5 text-xl md:text-2xl font-semibold leading-snug text-purple-900 antialiased">
                        "Growing in deen and dunya, together."
                    </h1>
                    <p class="mt-5 text-base md:text-lg leading-relaxed max-w-2xl text-foreground-600">
                        <span class="md:block">
                            "Merzah gathers your masjid, your neighbors, and your growth into one home for"
                        </span>
                        <span class="md:block">
                            "everyday faith. Join your community, stay close, and keep moving forward"
                        </span>
                        <span class="md:block">"in deen and dunya, together every day."</span>
                    </p>
                    <div class="mt-9 flex w-full flex-col items-stretch justify-center gap-4 sm:w-auto sm:flex-row sm:flex-wrap sm:items-center">
                        <a
                            href="#start"
                            class="inline-flex w-full items-center justify-center px-7 py-3.5 rounded-2xl bg-indigo-400 hover:bg-indigo-500 transition-colors duration-300 cursor-pointer text-white text-base font-bold shadow-lg shadow-indigo-500/25 sm:w-auto"
                        >
                            "Get started"
                        </a>
                        <a
                            href="#community"
                            class="inline-flex w-full items-center justify-center px-7 py-3.5 rounded-2xl bg-transparent border-indigo-400 border-2 hover:bg-indigo-200/30 transition-colors duration-300 cursor-pointer text-indigo-400 text-base font-bold sm:w-auto"
                        >
                            "Explore Merzah"
                        </a>
                    </div>
                </div>
            </header>

            // ======================= HIGHLIGHT CARDS =======================
            <section aria-label="What Merzah brings together" class="relative z-10">
                <div class="max-w-6xl mx-auto px-5 sm:px-8 -mt-24">
                    <div class="grid gap-5 sm:grid-cols-2 lg:grid-cols-3">
                        <div class="rounded-2xl bg-white p-6 ring-1 ring-indigo-950/10 shadow-[0_0.75rem_2rem_rgba(31,20,58,0.08)]">
                            <span class="w-11 h-11 inline-flex items-center justify-center rounded-xl bg-violet-50 ring-1 ring-violet-200/70">
                                <svg
                                    class="w-6 h-6 text-violet-700"
                                    viewBox="0 0 24 24"
                                    fill="none"
                                    stroke="currentColor"
                                    stroke-width="1.8"
                                >
                                    <circle cx="12" cy="12" r="8.5" />
                                    <path stroke-linecap="round" d="M12 7.5V12l3 2" />
                                </svg>
                            </span>
                            <h2 class="mt-5 text-lg font-bold tracking-tight leading-tight text-purple-900">
                                "Live Mosque Updates"
                            </h2>
                            <p class="mt-2 text-[15px] leading-relaxed text-gray-600">
                                "Live Adhan and Iqamah updates from your masjid."
                            </p>
                            <a
                                href="#start"
                                class="mt-4 inline-flex items-center text-sm font-bold text-violet-700 hover:text-violet-900"
                            >
                                "Get started " <span class="ml-1" aria-hidden="true">"→"</span>
                            </a>
                        </div>
                        <div class="rounded-2xl bg-white p-6 ring-1 ring-indigo-950/10 shadow-[0_0.75rem_2rem_rgba(31,20,58,0.08)]">
                            <span class="w-11 h-11 inline-flex items-center justify-center rounded-xl bg-violet-50 ring-1 ring-violet-200/70">
                                <svg
                                    class="w-6 h-6 text-violet-700"
                                    viewBox="0 0 24 24"
                                    fill="none"
                                    stroke="currentColor"
                                    stroke-width="1.8"
                                >
                                    <rect x="4" y="5.5" width="16" height="14" rx="2" />
                                    <path stroke-linecap="round" d="M4 10h16M8.5 3.5v4M15.5 3.5v4" />
                                </svg>
                            </span>
                            <h2 class="mt-5 text-lg font-bold tracking-tight leading-tight text-purple-900">
                                "Community Events"
                            </h2>
                            <p class="mt-2 text-[15px] leading-relaxed text-gray-600">
                                "Halaqahs, lectures, workshops, and local gatherings."
                            </p>
                            <a
                                href="#community"
                                class="mt-4 inline-flex items-center text-sm font-bold text-violet-700 hover:text-violet-900"
                            >
                                "Browse events " <span class="ml-1" aria-hidden="true">"→"</span>
                            </a>
                        </div>
                        <div class="rounded-2xl bg-white p-6 ring-1 ring-indigo-950/10 shadow-[0_0.75rem_2rem_rgba(31,20,58,0.08)] sm:col-span-2 lg:col-span-1">
                            <span class="w-11 h-11 inline-flex items-center justify-center rounded-xl bg-violet-50 ring-1 ring-violet-200/70">
                                <svg
                                    class="w-6 h-6 text-violet-700"
                                    viewBox="0 0 24 24"
                                    fill="none"
                                    stroke="currentColor"
                                    stroke-width="1.8"
                                    stroke-linejoin="round"
                                >
                                    <path
                                        stroke-linecap="round"
                                        d="M12 6.5C10.5 5.5 8.7 5 6.5 5c-1 0-1.9.1-2.5.3v13.2c.6-.2 1.5-.3 2.5-.3 2.2 0 4 .5 5.5 1.5 1.5-1 3.3-1.5 5.5-1.5 1 0 1.9.1 2.5.3V5.3c-.6-.2-1.5-.3-2.5-.3-2.2 0-4 .5-5.5 1.5zm0 0v13.2"
                                    />
                                </svg>
                            </span>
                            <h2 class="mt-5 text-lg font-bold tracking-tight leading-tight text-purple-900">
                                "Learn for Deen and Dunya"
                            </h2>
                            <p class="mt-2 text-[15px] leading-relaxed text-gray-600">
                                "Guided paths in Islamic knowledge, career, and life skills."
                            </p>
                            <a
                                href="#community"
                                class="mt-4 inline-flex items-center text-sm font-bold text-violet-700 hover:text-violet-900"
                            >
                                "Explore paths " <span class="ml-1" aria-hidden="true">"→"</span>
                            </a>
                        </div>
                    </div>
                </div>
            </section>

            // ===================== COMMUNITY STATEMENT =====================
            <section id="community" class="relative py-16 sm:py-24">
                <div class="max-w-6xl mx-auto px-5 sm:px-8 grid lg:grid-cols-2 gap-10 lg:gap-12 items-start">
                    <div>
                        <span class="inline-flex rounded-md bg-violet-50 px-3 py-1.5 text-xs font-bold uppercase tracking-wide text-violet-700 ring-1 ring-violet-200/70">
                            "How it works"
                        </span>
                        <h2 class="mt-4 text-3xl sm:text-4xl xl:text-5xl font-bold tracking-tight leading-tight text-purple-900">
                            "Start in minutes."
                        </h2>
                        <p class="mt-6 text-base sm:text-lg leading-relaxed max-w-xl text-foreground-600">
                            "No setup, no noise. Just your masjid and community in one place."
                        </p>
                        <ul class="mt-8 space-y-4 text-[15px]">
                            <li class="flex gap-3">
                                <svg
                                    class="w-5 h-5 mt-0.5 shrink-0 text-[#d4a500]"
                                    viewBox="0 0 24 24"
                                    fill="none"
                                    stroke="currentColor"
                                    stroke-width="1.8"
                                    stroke-linejoin="round"
                                >
                                    <rect x="7" y="7" width="10" height="10" rx="1" />
                                    <rect x="7" y="7" width="10" height="10" rx="1" transform="rotate(45 12 12)" />
                                    <circle cx="12" cy="12" r="1.2" fill="currentColor" stroke="none" />
                                </svg>
                                <span>
                                    <strong class="font-bold text-purple-900">"Follow your masjid."</strong>
                                    <span class="text-gray-600">
                                        " Get live Adhan and Iqamah updates straight from your imam."
                                    </span>
                                </span>
                            </li>
                            <li class="flex gap-3">
                                <svg
                                    class="w-5 h-5 mt-0.5 shrink-0 text-[#d4a500]"
                                    viewBox="0 0 24 24"
                                    fill="none"
                                    stroke="currentColor"
                                    stroke-width="1.8"
                                    stroke-linejoin="round"
                                >
                                    <rect x="7" y="7" width="10" height="10" rx="1" />
                                    <rect x="7" y="7" width="10" height="10" rx="1" transform="rotate(45 12 12)" />
                                    <circle cx="12" cy="12" r="1.2" fill="currentColor" stroke="none" />
                                </svg>
                                <span>
                                    <strong class="font-bold text-purple-900">"Join the week."</strong>
                                    <span class="text-gray-600">
                                        " Find halaqahs, classes, and gatherings near you and show up."
                                    </span>
                                </span>
                            </li>
                            <li class="flex gap-3">
                                <svg
                                    class="w-5 h-5 mt-0.5 shrink-0 text-[#d4a500]"
                                    viewBox="0 0 24 24"
                                    fill="none"
                                    stroke="currentColor"
                                    stroke-width="1.8"
                                    stroke-linejoin="round"
                                >
                                    <rect x="7" y="7" width="10" height="10" rx="1" />
                                    <rect x="7" y="7" width="10" height="10" rx="1" transform="rotate(45 12 12)" />
                                    <circle cx="12" cy="12" r="1.2" fill="currentColor" stroke="none" />
                                </svg>
                                <span>
                                    <strong class="font-bold text-purple-900">"Keep growing."</strong>
                                    <span class="text-gray-600">
                                        " Follow guided paths for Quran, family, work, and life skills."
                                    </span>
                                </span>
                            </li>
                        </ul>
                    </div>
                    <div class="rounded-2xl bg-white p-6 sm:p-8 ring-1 ring-indigo-950/10 shadow-lg">
                        <span class="inline-flex rounded-md bg-violet-50 px-2 py-0.5 text-[0.65rem] font-semibold uppercase tracking-[0.14em] text-violet-700 ring-1 ring-violet-200/70">
                            "From the founder"
                        </span>
                        <p class="font-serif mt-4 text-xl sm:text-2xl leading-snug font-bold text-foreground-900">
                            "“If we want a better future for Muslims, we must excel first in deen, then in dunya. I started Merzah to help us grow in both, together.”"
                        </p>
                    </div>
                </div>
            </section>

            // ======================= FINAL CALL TO ACTION =======================
            <section id="start" class="relative overflow-hidden px-4 pb-4 sm:px-6 sm:pb-6 bg-surface-800">
                <div class="relative overflow-hidden max-w-6xl mx-auto rounded-3xl sm:rounded-4xl border border-violet-900/20 px-5 py-10 sm:px-8 sm:py-14 md:p-14 text-white shadow-[0_1.5rem_4rem_rgba(37,16,79,0.22)] bg-[radial-gradient(circle_at_top,rgba(255,255,255,0.16),transparent_28%),linear-gradient(135deg,#23104a_0%,#31105d_48%,#24103f_100%)]">
                    <div class="pointer-events-none absolute right-0 top-0 h-full w-full bg-[radial-gradient(circle_at_85%_25%,rgba(255,219,137,0.12),transparent_18%),radial-gradient(circle_at_18%_18%,rgba(255,255,255,0.12),transparent_14%)]"></div>
                    <div class="pointer-events-none absolute inset-x-0 bottom-0 h-12 opacity-30 mix-blend-soft-light">
                        <svg
                            viewBox="0 0 400 50"
                            preserveAspectRatio="xMidYMax slice"
                            class="w-full h-full fill-white"
                        >
                            <path d="M0,50 L0,40 L20,40 L20,32 L25,28 L30,32 L30,40 L50,40 L50,50 Z M70,50 L70,35 L80,35 L80,22 L90,12 L100,22 L100,35 L110,35 L110,50 Z M130,50 L130,42 L140,42 L140,38 L145,34 L150,38 L150,42 L160,42 L160,50 Z M180,50 L180,30 L190,30 L190,18 Q205,2 220,18 L220,30 L230,30 L230,50 Z M200,50 L200,38 Q200,30 210,30 Q220,30 220,38 L220,50 Z M250,50 L250,42 L260,42 L260,36 L265,32 L270,36 L270,42 L280,42 L280,50 Z M300,50 L300,35 L310,35 L310,26 L320,18 L330,26 L330,35 L340,35 L340,50 Z M360,50 L360,44 L370,44 L370,48 L380,48 L380,50 Z M385,50 L385,46 L395,46 L395,50 Z" />
                        </svg>
                    </div>

                    <div class="relative">
                        <span class="inline-flex items-center rounded-full border border-white/15 bg-white/10 px-4 py-2 text-sm font-medium text-violet-50">
                            "For mosque admins"
                        </span>
                        <h2 class="mt-5 text-3xl sm:text-4xl xl:text-5xl font-bold tracking-tight leading-tight text-white">
                            "Run your mosque on Merzah."
                        </h2>
                        <p class="mt-5 text-base sm:text-lg leading-relaxed text-violet-100/75 max-w-2xl">
                            "Become admin for your mosque: keep Adhan and Iqamah times updated, list your imam and muazzin, and publish events with RSVPs, all in one place."
                        </p>
                        <div class="mt-9 flex w-full flex-col items-stretch gap-4 sm:w-auto sm:flex-row sm:flex-wrap sm:items-center">
                            <a
                                href="#start"
                                class="inline-flex w-full items-center justify-center px-6 py-4 sm:w-auto sm:px-8 rounded-xl font-bold transition hover:brightness-105 text-center text-[15px] sm:text-base bg-[#fde68a] text-[#23104a]"
                            >
                                "Become a mosque admin " <span class="ml-2" aria-hidden="true">"→"</span>
                            </a>
                            <a
                                href="#community"
                                class="inline-flex w-full items-center justify-center px-6 py-3.5 sm:w-auto sm:px-7 rounded-xl border border-white/15 bg-white/10 text-white text-sm font-bold transition hover:bg-white/15"
                            >
                                "See how it works"
                            </a>
                        </div>
                    </div>
                </div>
            </section>
        </div>
    }
}
