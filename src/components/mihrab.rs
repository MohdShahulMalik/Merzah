use leptos::prelude::*;

/// Hidden SVG definitions shared by pages using the Mihrab design:
/// mihrab-arched clip paths and the 8-fold star lattice pattern.
#[component]
pub fn MihrabDefs() -> impl IntoView {
    view! {
        <svg width="0" height="0" aria-hidden="true" style="position:absolute">
            <defs>
                <clipPath id="mihrab" clipPathUnits="objectBoundingBox">
                    <path d="M0,1 L0,0.42 C0,0.17 0.2,0 0.5,0 C0.8,0 1,0.17 1,0.42 L1,1 Z" />
                </clipPath>
                <clipPath id="mihrab-wide" clipPathUnits="objectBoundingBox">
                    <path d="M0,1 L0,0.3 C0,0.1 0.22,0 0.5,0 C0.78,0 1,0.1 1,0.3 L1,1 Z" />
                </clipPath>
                <clipPath id="mihrab-niche" clipPathUnits="objectBoundingBox">
                    <path d="M0,1 L0,0.42 C0,0.17 0.2,0 0.5,0 C0.8,0 1,0.17 1,0.42 L1,1 Z" />
                </clipPath>
                <pattern id="mihrab-lattice" width="48" height="48" patternUnits="userSpaceOnUse">
                    <g fill="none" stroke="currentColor" stroke-width="1">
                        <rect x="12" y="12" width="24" height="24" />
                        <rect x="12" y="12" width="24" height="24" transform="rotate(45 24 24)" />
                        <circle cx="24" cy="24" r="7" />
                        <path d="M24 0v8M24 40v8M0 24h8M40 24h8" />
                    </g>
                </pattern>
            </defs>
        </svg>
    }
}

/// Small eight-rayed star (✦) used as the current-lesson medallion and badge glyph.
#[component]
pub fn MihrabStar(#[prop(into, optional)] class: Option<&'static str>) -> impl IntoView {
    let class = class.unwrap_or("w-4.5 h-4.5");

    view! {
        <svg
            class=class
            viewBox="0 0 24 24"
            fill="currentColor"
            aria-hidden="true"
        >
            <path d="M12 1.8l2.6 6.1 6.6.5-5 4.3 1.6 6.5L12 15.9 6.2 19.2l1.6-6.5-5-4.3 6.6-.5z" />
        </svg>
    }
}
