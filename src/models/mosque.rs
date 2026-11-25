use garde::Validate;
use serde::Deserialize;
use serde::Serialize;
#[cfg(feature = "ssr")]
use surrealdb::types::{RecordId, ToSql};
#[cfg(feature = "ssr")]
use surrealdb::types::{Geometry, SurrealValue};

#[cfg(feature = "ssr")]
use crate::models::api_responses::MosqueResponse;
#[cfg(feature = "ssr")]
use crate::models::user::User;

#[cfg(feature = "ssr")]
#[derive(Debug, Serialize, Deserialize, SurrealValue)]
pub struct MosqueFromOverpass {
    pub id: RecordId,
    pub name: Option<String>,
    pub location: Geometry,
    pub street: Option<String>,
    pub city: Option<String>,
    pub cover_img: Option<String>,
}

/// The payload an app admin sends to approve the addition of a new mosque.
///
/// It is the input of the `add-mosque` server function, so it has to be
/// serializable on the client side as well.
#[derive(Debug, Validate, Deserialize, Serialize, Clone)]
pub struct NewMosque {
    #[garde(length(min = 2, max = 100))]
    pub name: String,
    #[garde(range(min = -90.0, max = 90.0))]
    pub lat: f64,
    #[garde(range(min = -180.0, max = 180.0))]
    pub lon: f64,
    #[garde(length(min = 1, max = 200))]
    pub street: Option<String>,
    #[garde(length(min = 1, max = 100))]
    pub city: Option<String>,
    #[garde(length(min = 1, max = 500))]
    pub cover_img: Option<String>,
}

impl NewMosque {
    pub fn new(
        name: String,
        lat: f64,
        lon: f64,
        street: Option<String>,
        city: Option<String>,
        cover_img: Option<String>,
    ) -> Self {
        Self {
            name,
            lat,
            lon,
            street,
            city,
            cover_img,
        }
    }
}

/// The shape of a mosque record that is written to the database.
#[cfg(feature = "ssr")]
#[derive(Debug, Serialize, Deserialize, SurrealValue)]
pub struct MosqueInsert {
    pub id: RecordId,
    pub name: Option<String>,
    pub location: Geometry,
    pub street: Option<String>,
    pub city: Option<String>,
    pub cover_img: Option<String>,
}

#[cfg(feature = "ssr")]
impl MosqueInsert {
    pub fn new(
        id: RecordId,
        name: Option<String>,
        location: Geometry,
        street: Option<String>,
        city: Option<String>,
        cover_img: Option<String>,
    ) -> Self {
        Self {
            id,
            name,
            location,
            street,
            city,
            cover_img,
        }
    }
}

#[cfg(feature = "ssr")]
#[derive(Debug, Deserialize, Serialize, SurrealValue)]
pub struct MosqueSearchResult {
    pub id: RecordId,
    pub location: Geometry,
    pub name: Option<String>,
    pub street: Option<String>,
    pub city: Option<String>,
    pub adhan_times: Option<PrayerTimes>,
    pub jamat_times: Option<PrayerTimes>,
    pub imam: Option<User>,
    pub muazzin: Option<User>,
    pub cover_img: Option<String>,
}

#[cfg(feature = "ssr")]
#[derive(Debug, Deserialize, SurrealValue)]
pub struct MosqueRecord {
    pub id: RecordId,
    pub location: Geometry,
    pub name: Option<String>,
    pub street: Option<String>,
    pub city: Option<String>,
    pub cover_img: Option<String>,
    pub adhan_times: Option<PrayerTimes>,
    pub jamat_times: Option<PrayerTimes>,
}

/// Converts a stored point geometry to the `(lat, lon)` pair the client
/// structs expect. SurrealDB stores `(x, y)` which corresponds to
/// `(lon, lat)`.
#[cfg(feature = "ssr")]
fn point_to_lat_lon(geometry: &Geometry) -> (f64, f64) {
    match geometry {
        Geometry::Point(point) => (point.y(), point.x()),
        _ => (0.0, 0.0),
    }
}

#[cfg(feature = "ssr")]
impl MosqueSearchResult {
    pub fn from(self) -> MosqueResponse {
        let imam = self.imam.map(|imam| imam.into());
        let muazzin = self.muazzin.map(|muazzin| muazzin.into());

        MosqueResponse {
            id: self.id.to_sql(),
            location: point_to_lat_lon(&self.location),
            name: self.name,
            street: self.street,
            city: self.city,
            cover_img: self.cover_img,
            adhan_times: self.adhan_times,
            jamat_times: self.jamat_times,
            imam_contact: vec![],
            muazzin_contact: vec![],
            imam,
            muazzin,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct OverpassResponse {
    pub elements: Vec<MosqueElement>,
}

#[derive(Debug, Deserialize)]
pub struct MosqueElement {
    #[serde(rename = "type")]
    pub element_type: String,
    pub id: i64,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub center: Option<Center>,
    pub tags: Option<Tags>,
}

#[derive(Debug, Deserialize)]
pub struct Center {
    pub lat: f64,
    pub lon: f64,
}

#[derive(Debug, Deserialize)]
pub struct Tags {
    pub name: Option<String>,
    #[serde(rename = "addr:street")]
    pub street: Option<String>,
    #[serde(rename = "addr:city")]
    pub city: Option<String>,
    pub wikimedia_commons: Option<String>,
}

/// Prayer times stored in the database as strings ("HH:MM:SS" format)
/// Use this for creating/updating prayer_times records
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "ssr", derive(SurrealValue))]
pub struct PrayerTimes {
    pub fajr: String,
    pub dhuhr: String,
    pub asr: String,
    pub maghrib: String,
    pub isha: String,
    /// Serialized as `jummah` to match the database schema
    /// (`schemas/mosques.surql` spells this field `jummah`).
    #[serde(rename = "jummah", alias = "jumuah")]
    pub jumuah: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "ssr", derive(SurrealValue))]
pub struct PrayerTimesUpdate {
    pub adhan_times: Option<PrayerTimes>,
    pub jamat_times: Option<PrayerTimes>,
}

#[cfg(feature = "ssr")]
#[derive(Debug, Deserialize)]
pub struct MosqueData {
    pub id: String,
    pub name: Option<String>,
    pub location: Geometry,
    pub street: Option<String>,
    pub city: Option<String>,
    pub adhan_times: Option<PrayerTimes>,
    pub jamat_times: Option<PrayerTimes>,
}
