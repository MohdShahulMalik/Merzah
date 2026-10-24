//! Popup dialogs rendered over the app shell.
//!
//! The mosque info popup is built on the native `<dialog>` element so that the
//! browser handles focus trapping, inertness of the page behind it and the
//! Escape key, instead of a hand rolled `div` overlay.

use chrono::NaiveTime;
use leptos::ev::MouseEvent;
use leptos::html::Dialog;
use leptos::prelude::*;
use leptos::web_sys::HtmlDialogElement;
use reactive_stores::OptionStoreExt;
use reactive_stores::Store;
use wasm_bindgen::JsCast;

use crate::app::AppState;
use crate::app::AppStateStoreFields;
use crate::components::cards::DEFAULT_MOSQUE_IMAGE;
use crate::models::api_responses::MixedMosqueResponse;
use crate::models::api_responses::MosqueResponse;
use crate::models::mosque::PrayerTimes;
use crate::models::nearby_mosque::UNNAMED_MOSQUE_NAME;
use crate::models::user::UserIdentifierOnClient;
use crate::models::user::UserOnClient;
use crate::server_functions::mosque::fetch_mosques_for_location;

/// A mosque info popup rendered as a native modal `<dialog>`.
///
/// The caller owns the `open` signal: setting it to `true` opens the dialog,
/// setting it to `false` closes it, and the dialog writes `false` back into it
/// whenever the user dismisses it with the Escape key, the close button or a
/// click on the backdrop.
///
/// Callers that already hold the mosque (map markers, nearby mosque cards)
/// pass it through `mosque`. Callers that only know the mosque by name (the
/// home page event cards) pass `mosque_name` instead and the dialog resolves
/// the mosque itself from the mosques around the user's current location.
#[component]
pub fn MosqueInfoDialog(
    open: RwSignal<bool>,
    #[prop(optional)] mosque: Signal<Option<MosqueResponse>>,
    #[prop(optional)] mosque_name: Signal<Option<String>>,
) -> impl IntoView {
    let dialog_ref = NodeRef::<Dialog>::new();
    let app_state = expect_context::<Store<AppState>>();

    let user_pos = move || app_state.coords().map(|coords| coords.read().as_lat_lon());

    // Nothing has to be fetched when the caller already holds the mosque, so
    // the source collapses to `(None, None)` and the resource resolves to
    // `None` without ever hitting the server.
    let resolve_source = move || {
        if mosque.get().is_some() {
            (None, None)
        } else {
            (user_pos(), mosque_name.get())
        }
    };

    let resolved = Resource::new(
        resolve_source,
        |(position, wanted_name): (Option<(f64, f64)>, Option<String>)| async move {
            let (lat, lon) = position?;
            let wanted_name = wanted_name?.trim().to_lowercase();

            if wanted_name.is_empty() {
                return None;
            }

            let mosques = fetch_mosques_for_location(lat, lon, false)
                .await
                .ok()
                .and_then(|response| response.data)
                .map(|mixed| match mixed {
                    MixedMosqueResponse::SingleMosque(mosque) => vec![mosque],
                    MixedMosqueResponse::MosquesVec(mosques) => mosques,
                })
                .unwrap_or_default();

            mosques.into_iter().find(|candidate| {
                candidate
                    .name
                    .clone()
                    .unwrap_or_else(|| UNNAMED_MOSQUE_NAME.to_string())
                    .trim()
                    .to_lowercase()
                    == wanted_name
            })
        },
    );

    // Keep the element in sync with the signal. `show_modal()` throws when the
    // dialog is already open and `close()` fires a spurious `close` event when
    // it is already closed, so both are guarded by the current state.
    Effect::new(move |_| {
        let should_be_open = open.get();

        if let Some(dialog) = dialog_ref.get() {
            if should_be_open {
                if !dialog.open() {
                    let _ = dialog.show_modal();
                }
            } else if dialog.open() {
                dialog.close();
            }
        }
    });

    // A click lands on the `<dialog>` itself only when it hit the backdrop:
    // every click inside the card bubbles up from one of its children, none of
    // which casts to an `HtmlDialogElement`.
    let on_backdrop_click = move |ev: MouseEvent| {
        let clicked_dialog = ev
            .target()
            .and_then(|target| target.dyn_into::<HtmlDialogElement>().ok())
            .is_some();

        if clicked_dialog {
            open.set(false);
        }
    };

    view! {
        <dialog
            node_ref=dialog_ref
            class="mosque-info-dialog m-auto w-[calc(100vw-2rem)] max-w-xl max-h-[85dvh] overflow-y-auto overflow-x-hidden scrollbar-hide rounded-2xl border border-stroke-muted bg-surface-700 text-foreground-900 shadow-2xl"
            on:close=move |_| open.set(false)
            on:click=on_backdrop_click
        >
            <div class="relative min-w-0">
                <div class="absolute right-3 top-3 z-10 p-1">
                    <button
                        type="button"
                        aria-label="Close mosque details"
                        class="flex h-9 w-9 items-center justify-center rounded-full border border-white/25 bg-white/18 text-white backdrop-blur-md transition-colors duration-300 ease-out hover:bg-white hover:text-[#050047]"
                        on:click=move |_| open.set(false)
                    >
                        <svg
                            class="h-4 w-4"
                            viewBox="0 0 24 24"
                            fill="none"
                            stroke="currentColor"
                            stroke-width="2"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            aria-hidden="true"
                        >
                            <path d="M18 6 6 18" />
                            <path d="m6 6 12 12" />
                        </svg>
                    </button>
                </div>

                <Suspense fallback=move || view! { <MosqueInfoSkeleton /> }>
                    {move || {
                        let data = mosque.get().or_else(|| resolved.get().flatten());

                        match data {
                            Some(mosque) => view! { <MosqueInfoCard mosque=mosque /> }.into_any(),
                            None => view! { <MosqueInfoEmptyState /> }.into_any(),
                        }
                    }}
                </Suspense>
            </div>
        </dialog>
    }
}

/// Shimmering placeholder shown while the mosque is being resolved by name.
#[component]
fn MosqueInfoSkeleton() -> impl IntoView {
    view! {
        <div class="min-w-0">
            <div class="h-44 w-full animate-pulse bg-[#c9cef3] dark:bg-white/5 sm:h-52"></div>

            <div class="space-y-4 p-4 sm:p-5">
                <div class="h-3 w-28 animate-pulse rounded-full bg-[#e8edff] dark:bg-white/5"></div>

                <div class="grid grid-cols-2 gap-2 sm:grid-cols-3">
                    {(0..6)
                        .map(|_| {
                            view! {
                                <div class="h-20 animate-pulse rounded-lg border border-stroke-muted bg-[#e8edff]/70 dark:bg-white/5"></div>
                            }
                        })
                        .collect::<Vec<_>>()}
                </div>

                <div class="h-12 animate-pulse rounded-lg border border-stroke-muted bg-[#e8edff]/70 dark:bg-white/5"></div>
            </div>
        </div>
    }
}

/// Shown when a mosque name could not be matched to any mosque nearby.
#[component]
fn MosqueInfoEmptyState() -> impl IntoView {
    view! {
        <div class="min-w-0 px-6 py-12 text-center">
            <div class="mx-auto grid h-12 w-12 place-items-center rounded-full bg-[#e8edff] text-[#211c74] dark:bg-sky-400/10 dark:text-sky-200">
                <svg
                    class="h-6 w-6"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="1.8"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    aria-hidden="true"
                >
                    <circle cx="12" cy="12" r="9" />
                    <path d="M12 8v4" />
                    <path d="M12 16h.01" />
                </svg>
            </div>

            <p class="mt-4 font-medium text-foreground-900">"Mosque details unavailable"</p>
            <p class="mt-1 text-sm text-foreground-600">
                "We could not find this mosque among the ones around you right now."
            </p>
        </div>
    }
}

/// The mosque card itself: hero, prayer times, personnel and the action row.
#[component]
fn MosqueInfoCard(mosque: MosqueResponse) -> impl IntoView {
    let MosqueResponse {
        location,
        name,
        street,
        city,
        cover_img,
        adhan_times,
        jamat_times,
        imam,
        muazzin,
        imam_contact,
        muazzin_contact,
        ..
    } = mosque;

    let (lat, lon) = location;
    let mosque_name = name.unwrap_or_else(|| UNNAMED_MOSQUE_NAME.to_string());
    let image_url = cover_img.unwrap_or_else(|| DEFAULT_MOSQUE_IMAGE.to_string());
    let image_alt = format!("{} cover", mosque_name);
    let coordinates = format!("{lat:.4}, {lon:.4}");
    let directions_url =
        format!("https://www.google.com/maps/dir/?api=1&destination={lat},{lon}");

    let location_line = match (street, city) {
        (Some(street), Some(city)) => Some(format!("{street} • {city}")),
        (Some(street), None) => Some(street),
        (None, Some(city)) => Some(city),
        (None, None) => None,
    };

    let cells = prayer_rows()
        .into_iter()
        .map(|(label, read_time)| {
            let jamat_text = format_time(jamat_times.as_ref().map(read_time));
            let (jamat_main, jamat_meridiem) = split_time_meridiem(&jamat_text);
            let adhan_line = adhan_times
                .as_ref()
                .map(|times| format!("Adhan {}", read_time(times).format("%-I:%M %p")));

            (label, jamat_main, jamat_meridiem, adhan_line)
        })
        .collect::<Vec<_>>();

    let has_personnel = imam.is_some()
        || muazzin.is_some()
        || !imam_contact.is_empty()
        || !muazzin_contact.is_empty();

    view! {
        <div class="min-w-0">
            <header class="relative h-44 overflow-hidden bg-[#c9cef3] sm:h-52">
                <img src=image_url alt=image_alt class="h-full w-full object-cover" />

                <div class="pointer-events-none absolute inset-0 bg-linear-to-t from-[#050047]/85 via-[#050047]/25 to-transparent"></div>

                <div class="pointer-events-none absolute inset-0">
                    <svg class="absolute top-[10%] left-[7%]" width="3" height="3"><circle cx="1.5" cy="1.5" r="1.5" fill="white" opacity="0.8"/></svg>
                    <svg class="absolute top-[18%] left-[19%]" width="4" height="4"><circle cx="2" cy="2" r="2" fill="white" opacity="0.6"/></svg>
                    <svg class="absolute top-[8%] left-[33%]" width="3" height="3"><circle cx="1.5" cy="1.5" r="1.5" fill="white" opacity="0.7"/></svg>
                    <svg class="absolute top-[24%] left-[47%]" width="1" height="1"><circle cx="0.5" cy="0.5" r="0.5" fill="white" opacity="0.9"/></svg>
                    <svg class="absolute top-[13%] left-[61%]" width="3" height="3"><circle cx="1.5" cy="1.5" r="1.5" fill="white" opacity="0.5"/></svg>
                    <svg class="absolute top-[6%] left-[74%]" width="4" height="4"><circle cx="2" cy="2" r="2" fill="white" opacity="0.6"/></svg>
                    <svg class="absolute top-[28%] left-[11%]" width="1" height="1"><circle cx="0.5" cy="0.5" r="0.5" fill="white" opacity="0.7"/></svg>
                    <svg class="absolute top-[16%] left-[86%]" width="3" height="3"><circle cx="1.5" cy="1.5" r="1.5" fill="white" opacity="0.8"/></svg>
                    <svg class="absolute top-[33%] left-[69%]" width="3" height="3"><circle cx="1.5" cy="1.5" r="1.5" fill="white" opacity="0.5"/></svg>
                    <svg class="absolute top-[38%] left-[41%]" width="1" height="1"><circle cx="0.5" cy="0.5" r="0.5" fill="white" opacity="0.4"/></svg>
                </div>

                <div class="absolute inset-x-0 bottom-0 min-w-0 space-y-2 px-4 pb-4 sm:px-5">
                    <h2 class="min-w-0 text-xl font-bold leading-tight text-white text-balance break-words sm:text-2xl">
                        {mosque_name}
                    </h2>

                    {location_line
                        .map(|line| {
                            view! {
                                <p class="flex min-w-0 items-center gap-1.5 text-sm text-sky-100/85">
                                    <svg
                                        class="h-3.5 w-3.5 shrink-0 text-[#f0bd25]"
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
                                    <span class="truncate">{line}</span>
                                </p>
                            }
                        })}

                    <span class="inline-flex rounded-full border border-white/15 bg-white/10 px-3 py-1 text-xs text-sky-100/85 backdrop-blur">
                        {coordinates}
                    </span>
                </div>
            </header>

            <div class="min-w-0 space-y-5 p-4 sm:p-5">
                <section class="min-w-0 space-y-3">
                    <div>
                        <p class="text-[0.65rem] font-semibold uppercase tracking-[0.22em] text-foreground-600">
                            "Prayer Times"
                        </p>
                        <span class="mt-1 block h-px w-8 bg-[#f0bd25]"></span>
                    </div>

                    <div class="grid grid-cols-2 gap-2 sm:grid-cols-3">
                        {cells
                            .into_iter()
                            .map(|(label, jamat_main, jamat_meridiem, adhan_line)| {
                                let is_jumuah = label == "Jumuah";
                                let cell_class = if is_jumuah {
                                    "rounded-lg border border-[#f0bd25]/50 bg-[#e8edff]/70 px-3 py-2.5 text-center dark:bg-white/5"
                                } else {
                                    "rounded-lg border border-stroke-muted bg-[#e8edff]/70 px-3 py-2.5 text-center dark:bg-white/5"
                                };

                                view! {
                                    <div class=cell_class>
                                        <p class="flex items-center justify-center gap-1 text-[0.6rem] uppercase tracking-[0.16em] text-foreground-600">
                                            {is_jumuah
                                                .then(|| {
                                                    view! {
                                                        <span class="inline-block h-1 w-1 rounded-full bg-[#f0bd25]"></span>
                                                    }
                                                })}
                                            <span>{label}</span>
                                        </p>

                                        <p class="mt-1.5 whitespace-nowrap text-base font-bold leading-none text-[#050047] dark:text-sky-200">
                                            {jamat_main}
                                        </p>

                                        {jamat_meridiem
                                            .map(|meridiem| {
                                                view! {
                                                    <p class="mt-1 text-[0.6rem] font-bold uppercase leading-none tracking-[0.22em] text-foreground-600">
                                                        {meridiem}
                                                    </p>
                                                }
                                            })}

                                        {adhan_line
                                            .map(|line| {
                                                view! {
                                                    <p class="mt-1.5 text-[0.7rem] text-foreground-600">{line}</p>
                                                }
                                            })}
                                    </div>
                                }
                            })
                            .collect::<Vec<_>>()}
                    </div>
                </section>

                {has_personnel
                    .then(|| {
                        view! {
                            <section class="min-w-0 space-y-3">
                                <div>
                                    <p class="text-[0.65rem] font-semibold uppercase tracking-[0.22em] text-foreground-600">
                                        "Personnel"
                                    </p>
                                    <span class="mt-1 block h-px w-8 bg-[#f0bd25]"></span>
                                </div>

                                {imam
                                    .map(|user| {
                                        view! {
                                            <PersonRow role_label="Imam" user=user contacts=imam_contact />
                                        }
                                    })}

                                {muazzin
                                    .map(|user| {
                                        view! {
                                            <PersonRow
                                                role_label="Muazzin"
                                                user=user
                                                contacts=muazzin_contact
                                            />
                                        }
                                    })}
                            </section>
                        }
                    })}

                <div class="flex min-w-0 flex-wrap items-center gap-3">
                    <a
                        href=directions_url
                        target="_blank"
                        rel="noopener noreferrer"
                        class="inline-flex min-h-[44px] items-center rounded-lg bg-indigo-400 px-4 py-2.5 text-sm font-medium text-white transition-colors duration-300 ease-out hover:bg-indigo-500"
                    >
                        "Directions →"
                    </a>
                </div>
            </div>
        </div>
    }
}

/// A single personnel row plus that person's contact pills.
#[component]
fn PersonRow(
    role_label: &'static str,
    user: UserOnClient,
    contacts: Vec<UserIdentifierOnClient>,
) -> impl IntoView {
    let display_name = user.display_name;

    view! {
        <div class="min-w-0 space-y-2">
            <div class="flex min-w-0 items-center gap-3 rounded-lg border border-stroke-muted px-3 py-2.5">
                <div class="grid h-9 w-9 shrink-0 place-items-center rounded-full bg-[#e8edff] text-[#211c74] dark:bg-sky-400/10 dark:text-sky-200">
                    <svg
                        class="h-5 w-5"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="1.8"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        aria-hidden="true"
                    >
                        <path d="M20 21a8 8 0 0 0-16 0" />
                        <circle cx="12" cy="7" r="4" />
                    </svg>
                </div>

                <p class="min-w-0 flex-1 truncate font-bold text-foreground-900">{display_name}</p>

                <span class="shrink-0 text-[0.6rem] uppercase tracking-[0.16em] text-foreground-600">
                    {role_label}
                </span>
            </div>

            {(!contacts.is_empty())
                .then(|| {
                    view! {
                        <div class="flex min-w-0 flex-wrap gap-2">
                            {contacts
                                .into_iter()
                                .map(|contact| {
                                    let pill = format!(
                                        "{}: {}",
                                        contact.identifier_type,
                                        contact.identifier_value,
                                    );

                                    view! {
                                        <span class="rounded-full bg-[#e8edff] px-2.5 py-1 text-[0.7rem] font-medium text-[#211c74] dark:bg-sky-400/10 dark:text-sky-200">
                                            {pill}
                                        </span>
                                    }
                                })
                                .collect::<Vec<_>>()}
                        </div>
                    }
                })}
        </div>
    }
}

/// The six prayers rendered by the times grid, each paired with the accessor
/// that reads it off a [`PrayerTimes`].
fn prayer_rows() -> [(&'static str, fn(&PrayerTimes) -> NaiveTime); 6] {
    [
        ("Fajr", |times| times.fajr),
        ("Dhuhr", |times| times.dhuhr),
        ("Asr", |times| times.asr),
        ("Maghrib", |times| times.maghrib),
        ("Isha", |times| times.isha),
        ("Jumuah", |times| times.jumuah),
    ]
}

/// Formats a prayer time, falling back to an em dash when the mosque has not
/// published that set of times.
fn format_time(time: Option<NaiveTime>) -> String {
    match time {
        Some(time) => time.format("%-I:%M %p").to_string(),
        None => "—".to_string(),
    }
}

/// Splits `"12:45 PM"` into `("12:45", Some("PM"))` so the large figure fits a
/// narrow grid cell with the meridiem stacked beneath it. Values without a
/// meridiem (such as the `"—"` placeholder) return `None` and render no extra
/// line.
fn split_time_meridiem(value: &str) -> (String, Option<String>) {
    match value.trim().rsplit_once(' ') {
        Some((main, meridiem)) => (main.to_string(), Some(meridiem.to_string())),
        None => (value.trim().to_string(), None),
    }
}
