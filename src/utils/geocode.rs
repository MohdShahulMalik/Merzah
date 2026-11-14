use crate::models::nominatim::NominatimReverse;

/// Reverse geocodes a coordinate into a short "City, ST" label (e.g.
/// "Detroit, MI") using the OpenStreetMap Nominatim reverse endpoint.
///
/// Returns `None` when the request fails or the response carries no usable
/// city; the caller then decides on a fallback label.
pub async fn reverse_geocode(latitude: f64, longitude: f64) -> Option<String> {
    let url = format!(
        "https://nominatim.openstreetmap.org/reverse?lat={latitude}&lon={longitude}&format=jsonv2&zoom=10&addressdetails=1"
    );

    let response = reqwest::get(&url).await.ok()?;
    let body: NominatimReverse = response.json().await.ok()?;

    let address = body.address;
    let city = address
        .city
        .or(address.town)
        .or(address.village)
        .or(address.municipality)
        .or(address.county)?;

    // For US addresses Nominatim provides "ISO3166-2-lvl4" like "US-MI";
    // fall back to the full state name when the abbreviation is missing.
    let state = address
        .iso_state
        .as_deref()
        .and_then(|code| code.rsplit('-').next().map(str::to_string))
        .or(address.state)
        .unwrap_or_default();

    if state.is_empty() {
        Some(city)
    } else {
        Some(format!("{city}, {state}"))
    }
}
