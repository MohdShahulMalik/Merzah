use leptos::prelude::*;

// Inline stroke icons (24x24 grid, 2px stroke, round caps/joins, currentColor).
// Path geometry comes from Lucide (https://lucide.dev, ISC license), the same
// set already used by the nav. MosqueIcon is hand-drawn to the same spec
// because Lucide has no mosque glyph. All icons are decorative; adjacent
// text carries the meaning, which is why every svg is aria-hidden.

#[component]
pub fn SearchIcon(#[prop(into, optional)] class: Option<&'static str>) -> impl IntoView {
    let class = class.unwrap_or("h-5 w-5");
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class aria-hidden="true">
            <path d="m21 21-4.34-4.34" />
            <circle cx="11" cy="11" r="8" />
        </svg>
    }
}

#[component]
pub fn SearchXIcon(#[prop(into, optional)] class: Option<&'static str>) -> impl IntoView {
    let class = class.unwrap_or("h-5 w-5");
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class aria-hidden="true">
            <path d="m13.5 8.5-5 5" />
            <path d="m8.5 8.5 5 5" />
            <circle cx="11" cy="11" r="8" />
            <path d="m21 21-4.3-4.3" />
        </svg>
    }
}

#[component]
pub fn MapPinIcon(#[prop(into, optional)] class: Option<&'static str>) -> impl IntoView {
    let class = class.unwrap_or("h-5 w-5");
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class aria-hidden="true">
            <path d="M20 10c0 4.993-5.539 10.193-7.399 11.799a1 1 0 0 1-1.202 0C9.539 20.193 4 14.993 4 10a8 8 0 0 1 16 0" />
            <circle cx="12" cy="10" r="3" />
        </svg>
    }
}

#[component]
pub fn ClockIcon(#[prop(into, optional)] class: Option<&'static str>) -> impl IntoView {
    let class = class.unwrap_or("h-5 w-5");
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class aria-hidden="true">
            <circle cx="12" cy="12" r="10" />
            <path d="M12 6v6l4 2" />
        </svg>
    }
}

#[component]
pub fn CalendarIcon(#[prop(into, optional)] class: Option<&'static str>) -> impl IntoView {
    let class = class.unwrap_or("h-5 w-5");
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class aria-hidden="true">
            <path d="M8 2v3" />
            <path d="M16 2v3" />
            <rect x="3" y="3" width="18" height="18" rx="2" />
            <path d="M3 9h18" />
        </svg>
    }
}

#[component]
pub fn BookOpenIcon(#[prop(into, optional)] class: Option<&'static str>) -> impl IntoView {
    let class = class.unwrap_or("h-5 w-5");
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class aria-hidden="true">
            <path d="M12 5v16" />
            <path d="M20.001 19A2 2 0 0022 17V5a2 2 0 00-1.999-2L16 3.002A5 5 0 0012 5a5 5 0 00-4-2H4a2 2 0 00-2 2v12a2 2 0 001.999 2H8a5 5 0 014 2 5 5 0 014-2z" />
        </svg>
    }
}

#[component]
pub fn BookIcon(#[prop(into, optional)] class: Option<&'static str>) -> impl IntoView {
    let class = class.unwrap_or("h-5 w-5");
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class aria-hidden="true">
            <path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H19a1 1 0 0 1 1 1v18a1 1 0 0 1-1 1H6.5a1 1 0 0 1 0-5H20" />
        </svg>
    }
}

#[component]
pub fn CircleCheckIcon(#[prop(into, optional)] class: Option<&'static str>) -> impl IntoView {
    let class = class.unwrap_or("h-5 w-5");
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class aria-hidden="true">
            <circle cx="12" cy="12" r="10" />
            <path d="m16 9-5.5 5.5L8 12" />
        </svg>
    }
}

#[component]
pub fn CompassIcon(#[prop(into, optional)] class: Option<&'static str>) -> impl IntoView {
    let class = class.unwrap_or("h-5 w-5");
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class aria-hidden="true">
            <circle cx="12" cy="12" r="10" />
            <path d="m16.24 7.76-1.804 5.411a2 2 0 0 1-1.265 1.265L7.76 16.24l1.804-5.411a2 2 0 0 1 1.265-1.265z" />
        </svg>
    }
}

#[component]
pub fn MapIcon(#[prop(into, optional)] class: Option<&'static str>) -> impl IntoView {
    let class = class.unwrap_or("h-5 w-5");
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class aria-hidden="true">
            <path d="M14.106 5.553a2 2 0 0 0 1.788 0l3.659-1.83A1 1 0 0 1 21 4.619v12.764a1 1 0 0 1-.553.894l-4.553 2.277a2 2 0 0 1-1.788 0l-4.212-2.106a2 2 0 0 0-1.788 0l-3.659 1.83A1 1 0 0 1 3 19.381V6.618a1 1 0 0 1 .553-.894l4.553-2.277a2 2 0 0 1 1.788 0z" />
            <path d="M15 5.764v15" />
            <path d="M9 3.236v15" />
        </svg>
    }
}

#[component]
pub fn RocketIcon(#[prop(into, optional)] class: Option<&'static str>) -> impl IntoView {
    let class = class.unwrap_or("h-5 w-5");
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class aria-hidden="true">
            <path d="M12 15v5s3.03-.55 4-2c1.08-1.62 0-5 0-5" />
            <path d="M4.5 16.5c-1.5 1.26-2 5-2 5s3.74-.5 5-2c.71-.84.7-2.13-.09-2.91a2.18 2.18 0 0 0-2.91-.09" />
            <path d="M9 12a22 22 0 0 1 2-3.95A12.88 12.88 0 0 1 22 2c0 2.72-.78 7.5-6 11a22.4 22.4 0 0 1-4 2z" />
            <path d="M9 12H4s.55-3.03 2-4c1.62-1.08 5 .05 5 .05" />
        </svg>
    }
}

#[component]
pub fn LibraryIcon(#[prop(into, optional)] class: Option<&'static str>) -> impl IntoView {
    let class = class.unwrap_or("h-5 w-5");
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class aria-hidden="true">
            <path d="m16 6 4 14" />
            <path d="M12 6v14" />
            <path d="M8 8v12" />
            <path d="M4 4v16" />
        </svg>
    }
}

#[component]
pub fn SparklesIcon(#[prop(into, optional)] class: Option<&'static str>) -> impl IntoView {
    let class = class.unwrap_or("h-5 w-5");
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class aria-hidden="true">
            <path d="M11.017 2.814a1 1 0 0 1 1.966 0l1.051 5.558a2 2 0 0 0 1.594 1.594l5.558 1.051a1 1 0 0 1 0 1.966l-5.558 1.051a2 2 0 0 0-1.594 1.594l-1.051 5.558a1 1 0 0 1-1.966 0l-1.051-5.558a2 2 0 0 0-1.594-1.594l-5.558-1.051a1 1 0 0 1 0-1.966l5.558-1.051a2 2 0 0 0 1.594-1.594z" />
            <path d="M20 2v4" />
            <path d="M22 4h-4" />
            <circle cx="4" cy="20" r="2" />
        </svg>
    }
}

#[component]
pub fn TargetIcon(#[prop(into, optional)] class: Option<&'static str>) -> impl IntoView {
    let class = class.unwrap_or("h-5 w-5");
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class aria-hidden="true">
            <circle cx="12" cy="12" r="10" />
            <circle cx="12" cy="12" r="6" />
            <circle cx="12" cy="12" r="2" />
        </svg>
    }
}

#[component]
pub fn LightbulbIcon(#[prop(into, optional)] class: Option<&'static str>) -> impl IntoView {
    let class = class.unwrap_or("h-5 w-5");
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class aria-hidden="true">
            <path d="M15 14c.2-1 .7-1.7 1.5-2.5 1-.9 1.5-2.2 1.5-3.5A6 6 0 0 0 6 8c0 1 .2 2.2 1.5 3.5.7.7 1.3 1.5 1.5 2.5" />
            <path d="M9 18h6" />
            <path d="M10 22h4" />
        </svg>
    }
}

#[component]
pub fn FileTextIcon(#[prop(into, optional)] class: Option<&'static str>) -> impl IntoView {
    let class = class.unwrap_or("h-5 w-5");
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class aria-hidden="true">
            <path d="M6 22a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.704.706l3.588 3.588A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2z" />
            <path d="M14 2v5a1 1 0 0 0 1 1h5" />
            <path d="M10 9H8" />
            <path d="M16 13H8" />
            <path d="M16 17H8" />
        </svg>
    }
}

#[component]
pub fn ClapperboardIcon(#[prop(into, optional)] class: Option<&'static str>) -> impl IntoView {
    let class = class.unwrap_or("h-5 w-5");
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class aria-hidden="true">
            <path d="m12.296 3.464 3.02 3.956" />
            <path d="M20.2 6 3 11l-.9-2.4c-.3-1.1.3-2.2 1.3-2.5l13.5-4c1.1-.3 2.2.3 2.5 1.3z" />
            <path d="M3 11h18v8a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z" />
            <path d="m6.18 5.276 3.1 3.899" />
        </svg>
    }
}

#[component]
pub fn HeadphonesIcon(#[prop(into, optional)] class: Option<&'static str>) -> impl IntoView {
    let class = class.unwrap_or("h-5 w-5");
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class aria-hidden="true">
            <path d="M3 14h3a2 2 0 0 1 2 2v3a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-7a9 9 0 0 1 18 0v7a2 2 0 0 1-2 2h-1a2 2 0 0 1-2-2v-3a2 2 0 0 1 2-2h3" />
        </svg>
    }
}

#[component]
pub fn ExternalLinkIcon(#[prop(into, optional)] class: Option<&'static str>) -> impl IntoView {
    let class = class.unwrap_or("h-5 w-5");
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class aria-hidden="true">
            <path d="M15 3h6v6" />
            <path d="M10 14 21 3" />
            <path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6" />
        </svg>
    }
}

#[component]
pub fn PuzzleIcon(#[prop(into, optional)] class: Option<&'static str>) -> impl IntoView {
    let class = class.unwrap_or("h-5 w-5");
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class aria-hidden="true">
            <path d="M15.39 4.39a1 1 0 0 0 1.68-.474 2.5 2.5 0 1 1 3.014 3.015 1 1 0 0 0-.474 1.68l1.683 1.682a2.414 2.414 0 0 1 0 3.414L19.61 15.39a1 1 0 0 1-1.68-.474 2.5 2.5 0 1 0-3.014 3.015 1 1 0 0 1 .474 1.68l-1.683 1.682a2.414 2.414 0 0 1-3.414 0L8.61 19.61a1 1 0 0 0-1.68.474 2.5 2.5 0 1 1-3.014-3.015 1 1 0 0 0 .474-1.68l-1.683-1.682a2.414 2.414 0 0 1 0-3.414L4.39 8.61a1 1 0 0 1 1.68.474 2.5 2.5 0 1 0 3.014-3.015 1 1 0 0 1-.474-1.68l1.683-1.682a2.414 2.414 0 0 1 3.414 0z" />
        </svg>
    }
}

#[component]
pub fn InboxIcon(#[prop(into, optional)] class: Option<&'static str>) -> impl IntoView {
    let class = class.unwrap_or("h-5 w-5");
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class aria-hidden="true">
            <polyline points="22 12 16 12 14 15 10 15 8 12 2 12" />
            <path d="M5.45 5.11 2 12v6a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-6l-3.45-6.89A2 2 0 0 0 16.76 4H7.24a2 2 0 0 0-1.79 1.11z" />
        </svg>
    }
}

#[component]
pub fn SproutIcon(#[prop(into, optional)] class: Option<&'static str>) -> impl IntoView {
    let class = class.unwrap_or("h-5 w-5");
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class aria-hidden="true">
            <path d="M14 9.536V7a4 4 0 0 1 4-4h1.5a.5.5 0 0 1 .5.5V5a4 4 0 0 1-4 4 4 4 0 0 0-4 4c0 2 1 3 1 5a5 5 0 0 1-1 3" />
            <path d="M4 9a5 5 0 0 1 8 4 5 5 0 0 1-8-4" />
            <path d="M5 21h14" />
        </svg>
    }
}

// Lucide has no mosque glyph; drawn to the same spec (24 grid, 2px stroke,
// round caps/joins). Dome, finial, two minarets, arched door.
#[component]
pub fn MosqueIcon(#[prop(into, optional)] class: Option<&'static str>) -> impl IntoView {
    let class = class.unwrap_or("h-5 w-5");
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class=class aria-hidden="true">
            <circle cx="12" cy="2.75" r="0.75" />
            <path d="M12 3.5V7" />
            <path d="M6.5 12.5a5.5 5.5 0 0 1 11 0" />
            <path d="M6.5 20v-7.5h11V20" />
            <path d="M3 20v-9.5h2.5V20" />
            <path d="M2.25 10.5 4.25 8l2 2.5" />
            <path d="M18.5 20v-9.5H21V20" />
            <path d="M17.75 10.5 19.75 8l2 2.5" />
            <path d="M10.25 20v-3.25a1.75 1.75 0 0 1 3.5 0V20" />
            <path d="M2 20h20" />
        </svg>
    }
}
