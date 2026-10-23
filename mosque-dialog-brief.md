TASK: Build a mosque info popup modal component for this repo. It is a Leptos 0.8.9 (SSR/CSR) app styled with Tailwind CSS v4 (CSS-first config, no tailwind.config.js — all tokens live in style/main.css). Write ONLY the component described below; do NOT wire it into any page, section, or card file — the caller wires it afterwards.

SCOPE: You may edit exactly two files and nothing else:
1. src/components/modal.rs  (currently an empty placeholder, already declared in src/components/mod.rs as `pub mod modal;`)
2. style/main.css  (only to append the dialog backdrop / open-animation CSS described below)

Do not add dependencies. Do not touch any .env, .surrealdb, or scripts/.auth_state files. When done, run `cargo check --features csr` and fix any errors in the code you wrote. Finish with a short summary of what you wrote.

====================
FILE 1: src/components/modal.rs
====================

Create a component named `MosqueInfoDialog` built on the NATIVE HTML <dialog> element (not a div overlay). The existing empty module is the perfect home for it.

Public API — must match EXACTLY (the caller already knows this signature):

#[component]
pub fn MosqueInfoDialog(
    open: RwSignal<bool>,
    #[prop(optional)] mosque: Signal<Option<MosqueResponse>>,
    #[prop(optional)] mosque_name: Signal<Option<String>>,
) -> impl IntoView

- `open` controls the dialog. An Effect watches it: when true call show_modal() on the dialog element (via NodeRef<leptos::html::Dialog>, which derefs to web_sys::HtmlDialogElement); when false call close(). Set `open.set(false)` in an `on:close` handler so Escape-key closes (dialog fires `close` natively) stay in sync with the signal.
- Backdrop click also closes: on the <dialog> element use `on:click` and close only when the click target IS the dialog element itself (not bubbled from children).
- `mosque` is the fully-loaded mosque (used by map markers and nearby cards where the caller already holds the data).
- When `mosque` is None but `mosque_name` is Some (this is the homepage event-card case, which only knows the mosque's name), resolve it yourself: read the global app state with `expect_context::<Store<AppState>>()` (see src/pages/mosques.rs lines 19-24 for the exact coords pattern: `app_state.coords().map(|coords| coords.read().as_lat_lon())`), then call the existing server function `fetch_mosques_for_location(lat, lon, false)` (crate::server_functions::mosque) and pick the response whose `name` matches `mosque_name` case-insensitively (trimmed). Treat a mosque whose name is None as matching the const `UNNAMED_MOSQUE_NAME` (crate::models::nearby_mosque). Use a leptos `Resource`/`Suspense` for the async part; while resolving show a skeleton shimmer inside the card; if nothing matched, show a friendly "Mosque details unavailable" empty state inside the card.

Data you will render — import from the top with `use` statements (repo convention: never inline `crate::` paths mid-code):
- `MosqueResponse` (crate::models::api_responses): { id: String, location: (f64,f64), name: Option<String>, street: Option<String>, city: Option<String>, cover_img: Option<String>, adhan_times: Option<PrayerTimes>, jamat_times: Option<PrayerTimes>, imam: Option<UserOnClient>, muazzin: Option<UserOnClient>, imam_contact: Vec<UserIdentifierOnClient>, muazzin_contact: Vec<UserIdentifierOnClient> }
- `PrayerTimes` (crate::models::mosque): fields fajr, dhuhr, asr, maghrib, isha, jumuah — all chrono::NaiveTime. Format times with `.format("%-I:%M %p")` and split main figure / meridiem for stacked typography (see `split_time_meridiem` in src/components/cards/home.rs).
- `UserOnClient` (crate::models::user): { id, display_name, role }.
- `UserIdentifierOnClient` (crate::models::user): { identifier_type, identifier_value }.
- `DEFAULT_MOSQUE_IMAGE` const — lives in src/components/cards/home.rs, check src/components/cards/mod.rs for the correct module path to import it from. Use it as the cover-image fallback.
- `UNNAMED_MOSQUE_NAME` (crate::models::nearby_mosque) fallback for a missing name.

Card layout (in this order inside the <dialog>, which itself IS the card):

1. HERO HEADER (relative, h-44 sm:h-52):
   - Cover image (cover_img or DEFAULT_MOSQUE_IMAGE) object-cover full-bleed.
   - Bottom-heavy gradient scrim: bg-linear-to-t from-[#050047]/85 via-[#050047]/25 to-transparent (this deep-navy + gold language is already used by NearbyMosqueCard in src/components/cards/home.rs — keep it consistent).
   - A "starfield": ~10 tiny absolutely-positioned white svg circles at scattered positions with opacities 0.4-0.9 — copy that technique from NextPrayerReminderCard in the same file.
   - Mosque name: large bold white, text-balance, bottom-left over the scrim; below it a location line with a small gold (#f0bd25) map-pin svg + street and city ("street • city", gracefully omitting whichever is None).
   - A small coordinates pill under the location: inline-flex rounded-full border border-white/15 bg-white/10 px-3 py-1 text-xs text-sky-100/85 backdrop-blur showing "lat, lon" with 4 decimal places.

2. CLOSE BUTTON (absolute top-3 right-3, above the image):
   - h-9 w-9 rounded-full border border-white/25 bg-white/18 text-white backdrop-blur-md, an X svg icon, generous touch target (add p-1 wrapper or min-h-[44px] hit area), hover:bg-white hover:text-[#050047], on:click closes via `open` signal.

3. PRAYER TIMES GRID (padded section on the card surface):
   - A slim section heading style used in this app: tiny uppercase tracking weathered label, e.g. text-[0.65rem] font-semibold uppercase tracking-[0.22em] text-foreground-600 with a short gold underline accent.
   - grid grid-cols-2 gap-2 sm:grid-cols-3 of 6 cells: Fajr, Dhuhr, Asr, Maghrib, Isha, Jumuah.
   - Each cell: rounded-lg border border-stroke-muted bg-[#e8edff]/70 dark:bg-white/5 px-3 py-2.5 text-center; prayer name as a tiny uppercase label (text-[0.6rem] tracking-[0.16em] text-foreground-600); then the iqamah (jamat) time in bold text-[#050047] dark:text-sky-200 with the meridiem stacked beneath it as a tiny bold uppercase line (mirror the typography of PrayerCard in src/components/cards/home.rs); under it a small line "Adhan hh:mm" (text-[0.7rem] text-foreground-600) when adhan times exist. Render "—" for any time that is None. Jumuah cell gets a subtle distinction: border-[#f0bd25]/50 + tiny gold dot next to its label.

4. PERSONNEL & CONTACTS (only render the section if imam, muazzin, or any contacts exist):
   - Rows for Imam and Muazzin: rounded-lg border border-stroke-muted px-3 py-2.5 flex items-center gap-3; a small circular avatar-ish placeholder (h-9 w-9 rounded-full bg-[#e8edff] dark:bg-sky-400/10 grid place-items-center with a person svg, text-[#211c74] dark:text-sky-200); display_name bold text-foreground-900; role label tiny uppercase text-foreground-600 on the right.
   - Contacts as pills: rounded-full bg-[#e8edff] dark:bg-sky-400/10 px-2.5 py-1 text-[0.7rem] font-medium text-[#211c74] dark:text-sky-200, formatted "{identifier_type}: {identifier_value}".

5. FOOTER ACTION ROW:
   - One primary button: an <a> styled like the app's primary action (rounded-lg bg-purple-600 px-4 py-2.5 text-sm font-medium text-white hover:bg-purple-700, min-h-[44px]) labelled "Directions →" linking to https://www.google.com/maps/dir/?api=1&destination={lat},{lon} (target _blank).

Dialog element itself (the card): classes like
`mosque-info-dialog m-auto w-[calc(100vw-2rem)] max-w-xl max-h-[85dvh] overflow-y-auto overflow-x-hidden rounded-2xl border border-stroke-muted bg-surface-700 text-foreground-900 shadow-2xl`
The <dialog> default margin:auto centers it; do not fight that. Keep dark-mode working through `dark:` utilities (the repo uses system-preference dark: defined in style/main.css).

====================
FILE 2: style/main.css
====================

Append ONLY a new block (do not modify existing rules) guarded in one comment like the others in the file:

```css
/* Mosque info <dialog>: dimmed violet-tinted scrim + open animation */
.mosque-info-dialog { /* fade + scale-in when opened */ }
.mosque-info-dialog::backdrop {
  background: oklch(0.1 0.05 279 / 0.55);
  backdrop-filter: blur(6px) saturate(1.05);
}
```

Implement it properly: a ~260ms fade-and-scale-in animation on the dialog when it gains [open], a fading animation on ::backdrop, and `animation: none` inside the existing prefers-reduced-motion spirit (the file already has a global reduced-motion rule for transitions; add one for your animations). Match the file's comment style.

====================
CODE STYLE (repo conventions, see AGENTS.md)
====================
- Leptos 0.8 `view!` macro, `#[component]`, `pub fn`, all imports at top of file, never `crate::foo::bar::Baz` inline.
- No struct-literal construction from other modules when a `new()` associated fn exists.
- Keep helper private fns (like a meridiem splitter) local to modal.rs rather than refactoring other files.
- Write clean, readable Rust; document the component with a `///` doc comment like neighboring files do.
