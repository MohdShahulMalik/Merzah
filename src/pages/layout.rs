use leptos::prelude::*;
use leptos_router::components::Outlet;

use crate::components::nav::Nav;

#[component]
pub fn AppLayout() -> impl IntoView {
    view! {
        <div class="lg:grid lg:min-h-screen lg:grid-cols-[auto_1fr]">
            <Nav/>
            <main class="pb-20 lg:pb-0 ml-5">
                <Outlet/>
            </main>
        </div>
    }
}
