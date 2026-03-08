use leptos::IntoView;
use leptos::prelude::*;

use crate::components::nav::Nav;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Nav/>
        <div class="dashboard">
            <h1>"Dashboard"</h1>
            // Add your dashboard content here
        </div>
    }
}
