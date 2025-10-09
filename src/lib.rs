pub mod app;
pub mod auth;
pub mod database;
pub mod errors;
pub mod models;
pub mod utils;

#[cfg(feature = "ssr")]
pub mod server_functions;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
