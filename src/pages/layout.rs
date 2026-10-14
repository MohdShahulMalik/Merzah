use leptos::prelude::*;
use leptos_router::components::Outlet;

use crate::components::nav::Nav;

#[component]
pub fn AppLayout() -> impl IntoView {
    view! {
        <div class="lg:grid lg:min-h-screen lg:grid-cols-[16rem_minmax(0,1fr)]">
            <Nav/>
            <main class="mx-auto min-w-0 w-full max-w-6xl px-4 pb-24 sm:px-6 lg:pb-8 lg:pr-8">
                <Outlet/>
            </main>
        </div>
    }
}
