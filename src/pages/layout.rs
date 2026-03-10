use leptos::prelude::*;
use leptos_router::components::Outlet;

use crate::components::nav::Nav;

#[component]
pub fn AppLayout() -> impl IntoView {
    view! {
        <div class="app-shell">
            <Nav/>
            <section class="app-content">
                <Outlet/>
            </section>
        </div>
    }
}
