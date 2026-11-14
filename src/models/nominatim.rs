use serde::Deserialize;

/// Subset of the `address` object returned by the OpenStreetMap Nominatim
/// reverse geocoding endpoint (`format=jsonv2`, `addressdetails=1`).
///
/// Nominatim uses different keys depending on the size of the place, so
/// consumers should walk down the city → town → village → municipality →
/// county fallback chain.
#[derive(Debug, Deserialize)]
pub struct NominatimAddress {
    pub city: Option<String>,
    pub town: Option<String>,
    pub village: Option<String>,
    pub municipality: Option<String>,
    pub county: Option<String>,
    pub state: Option<String>,
    #[serde(rename = "ISO3166-2-lvl4")]
    pub iso_state: Option<String>,
}

/// Subset of the Nominatim reverse geocoding response.
#[derive(Debug, Deserialize)]
pub struct NominatimReverse {
    pub address: NominatimAddress,
}
