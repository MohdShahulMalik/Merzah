//! Browser tests for the location local-storage persistence in `LocationProvider`.
//!
//! These tests drive a real (headless, mocked) `navigator.geolocation` and a real
//! `window.localStorage`. They assert observable behavior only:
//!
//!   * a position reported by the geolocation watch ends up in the app store,
//!   * a remount (with geolocation that never reports) restores the position
//!     that was stored earlier, proving it round-tripped through local storage,
//!   * the latest reported position is the one that survives a remount,
//!   * nothing is restored when storage is empty,
//!   * nothing happens at all while there is no authenticated user.
//!
//! No storage key names, serialisation formats or codecs are referenced, so the
//! tests stay valid if the storage implementation changes.
//!
//! Run with:
//!   cargo test --target wasm32-unknown-unknown --features csr --test wasm_location_storage
#![cfg(target_arch = "wasm32")]

use std::time::Duration;

use gloo_timers::future::sleep;
use js_sys::Reflect;
use leptos::mount::mount_to;
use leptos::prelude::*;
use reactive_stores::Store;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

use merzah::app::{AppState, AppStateStoreFields};
use merzah::components::location::LocationProvider;
use merzah::models::location::GeoPosition;
use merzah::models::user::UserOnClient;

wasm_bindgen_test_configure!(run_in_browser);

const FIRST_POSITION: (f64, f64) = (23.8103, 90.4125);
const SECOND_POSITION: (f64, f64) = (21.4225, 39.8262);

fn authenticated_user() -> UserOnClient {
    UserOnClient::new(
        "test-user-id".to_string(),
        "Test User".to_string(),
        "user".to_string(),
    )
}

fn clear_local_storage() {
    if let Some(storage) = web_sys::window()
        .and_then(|window| window.local_storage().ok().flatten())
    {
        storage.clear().expect("failed to clear local storage");
    }
}

/// Replaces `navigator.geolocation` with a mock.
///
/// The mock captures the success callback passed to `watchPosition` on
/// `window.__merzah_geo_success` and never calls it on its own, so each test
/// controls exactly when (or whether) a position is reported.
fn install_geolocation_mock() {
    let window = web_sys::window().expect("no window");
    let navigator = window.navigator();

    let mock = js_sys::Object::new();

    let watch = js_sys::Function::new_with_args(
        "success, error, options",
        "window.__merzah_geo_success = success; return 1;",
    );
    let clear = js_sys::Function::new_with_args("id", "");

    Reflect::set(&mock, &"watchPosition".into(), watch.as_ref()).unwrap();
    Reflect::set(&mock, &"clearWatch".into(), clear.as_ref()).unwrap();

    let descriptor = js_sys::Object::new();
    Reflect::set(&descriptor, &"value".into(), &mock).unwrap();
    Reflect::set(&descriptor, &"configurable".into(), &true.into()).unwrap();
    Reflect::define_property(&navigator, &"geolocation".into(), &descriptor)
        .expect("failed to mock navigator.geolocation");
}

/// Reports a position to the mocked geolocation watch, like a browser would.
fn report_position(latitude: f64, longitude: f64) {
    let window = web_sys::window().expect("no window");
    let callback = Reflect::get(window.as_ref(), &"__merzah_geo_success".into())
        .expect("geolocation mock not installed");
    let callback: js_sys::Function = callback
        .dyn_into()
        .expect("geolocation mock did not capture a success callback");

    let coords = js_sys::Object::new();
    Reflect::set(&coords, &"latitude".into(), &latitude.into()).unwrap();
    Reflect::set(&coords, &"longitude".into(), &longitude.into()).unwrap();

    let position = js_sys::Object::new();
    Reflect::set(&position, &"coords".into(), &coords).unwrap();
    Reflect::set(
        &position,
        &"timestamp".into(),
        &1_700_000_000_000f64.into(),
    )
    .unwrap();

    callback.call1(&JsValue::NULL, &position).unwrap();
}

fn mount_location_provider(user: Option<UserOnClient>) -> (Store<AppState>, impl Drop) {
    let app_state = Store::new(AppState::new(user, None));

    let document = web_sys::window()
        .expect("no window")
        .document()
        .expect("no document");
    let element = document
        .create_element("div")
        .expect("failed to create element")
        .unchecked_into::<web_sys::HtmlElement>();
    document.body().expect("no body").append_child(&element).unwrap();

    let handle = mount_to(element, {
        let app_state = app_state;
        move || {
            provide_context(app_state);
            view! { <LocationProvider /> }
        }
    });

    (app_state, handle)
}

#[wasm_bindgen_test]
async fn persisted_position_is_restored_on_next_mount() {
    clear_local_storage();
    install_geolocation_mock();

    let (_state, first_mount) = mount_location_provider(Some(authenticated_user()));
    sleep(Duration::from_millis(100)).await;
    report_position(FIRST_POSITION.0, FIRST_POSITION.1);
    sleep(Duration::from_millis(300)).await;
    drop(first_mount);

    // Remount without reporting any position: the position in the store can
    // only have come from local storage.
    install_geolocation_mock();
    let (second_state, _second_mount) = mount_location_provider(Some(authenticated_user()));
    sleep(Duration::from_millis(300)).await;

    assert_eq!(
        second_state.coords().get(),
        Some(GeoPosition::new(FIRST_POSITION.0, FIRST_POSITION.1)),
    );
}

#[wasm_bindgen_test]
async fn latest_reported_position_is_the_one_persisted() {
    clear_local_storage();
    install_geolocation_mock();

    let (state, first_mount) = mount_location_provider(Some(authenticated_user()));
    sleep(Duration::from_millis(100)).await;
    report_position(FIRST_POSITION.0, FIRST_POSITION.1);
    sleep(Duration::from_millis(300)).await;
    assert_eq!(
        state.coords().get(),
        Some(GeoPosition::new(FIRST_POSITION.0, FIRST_POSITION.1)),
    );

    report_position(SECOND_POSITION.0, SECOND_POSITION.1);
    sleep(Duration::from_millis(300)).await;
    drop(first_mount);

    install_geolocation_mock();
    let (second_state, _second_mount) = mount_location_provider(Some(authenticated_user()));
    sleep(Duration::from_millis(300)).await;

    assert_eq!(
        second_state.coords().get(),
        Some(GeoPosition::new(SECOND_POSITION.0, SECOND_POSITION.1)),
    );
}

#[wasm_bindgen_test]
async fn nothing_is_restored_when_local_storage_is_empty() {
    clear_local_storage();
    install_geolocation_mock();

    let (state, _mount) = mount_location_provider(Some(authenticated_user()));
    sleep(Duration::from_millis(300)).await;

    assert_eq!(state.coords().get(), None);
}

#[wasm_bindgen_test]
async fn nothing_is_persisted_or_applied_without_an_authenticated_user() {
    clear_local_storage();
    install_geolocation_mock();

    let (state, first_mount) = mount_location_provider(None);
    sleep(Duration::from_millis(300)).await;
    assert_eq!(state.coords().get(), None);
    drop(first_mount);

    install_geolocation_mock();
    let (second_state, _second_mount) = mount_location_provider(Some(authenticated_user()));
    sleep(Duration::from_millis(300)).await;

    assert_eq!(second_state.coords().get(), None);
}
