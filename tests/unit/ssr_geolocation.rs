use leptos::prelude::*;

#[test]
fn geolocation_is_noop_on_server() {
    let owner = Owner::new();

    let coords = owner.with(|| {
        let geo = leptos_use::use_geolocation();

        geo.coords.get()
    });

    assert!(coords.is_none());
}
