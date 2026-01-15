use serde::Deserialize;
use serde::Serialize;
#[cfg(feature = "ssr")]
use surrealdb::RecordId;
#[cfg(feature = "ssr")]
use surrealdb::sql::Geometry;
use chrono::NaiveTime;

use crate::models::api_responses::MosqueApiResponse;

#[cfg(feature = "ssr")]
#[derive(Debug, Serialize, Deserialize)]
pub struct MosqueRecord {
    pub id: RecordId,
    pub name: Option<String>,
    pub location: Geometry,
    pub street: Option<String>,
    pub city: Option<String>,
}

#[cfg(feature = "ssr")]
#[derive(Debug, Deserialize, Serialize)]
pub struct MosqueDbRes {
    pub id: RecordId,
    #[cfg_attr(feature = "ssr", serde(deserialize_with = "deserialize_surreal_point"))]
    pub location: (f64, f64),
    pub name: Option<String>,
    pub street: Option<String>,
    pub city: Option<String>,
}

#[cfg(feature = "ssr")]
fn deserialize_surreal_point<'de, D>(deserializer: D) -> Result<(f64, f64), D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::Deserialize;

    #[derive(Deserialize)]
    struct SurrealPoint(SurrealCoord);

    #[derive(Deserialize)]
    struct SurrealCoord {
        x: f64,
        y: f64,
    }

    let point = SurrealPoint::deserialize(deserializer)?;
    // SurrealDB stores (x, y) which corresponds to (lon, lat)
    // We want to return (lat, lon)
    Ok((point.0.y, point.0.x))
}

#[cfg(feature = "ssr")]
impl MosqueDbRes {
    pub fn from(self) -> MosqueApiResponse {
        MosqueApiResponse { location: self.location, name: self.name, street: self.street, city: self.city }
    }
}

#[derive(Debug, Deserialize)]
pub struct MosquesResponse {
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
}

/// Prayer times stored in the database as strings ("HH:MM:SS" format)
/// Use this for creating/updating prayer_times records
#[cfg(feature = "ssr")]
#[derive(Debug, Serialize, Deserialize)]
pub struct PrayerTimes {
    pub id: RecordId,
    pub fajr: NaiveTime,
    pub dhuhr: NaiveTime,
    pub asr: NaiveTime,
    pub maghrib: NaiveTime,
    pub isha: NaiveTime,
    pub jummah: NaiveTime,
}

/// For creating new prayer times records (without id)
#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePrayerTimes {
    pub fajr: NaiveTime,
    pub dhuhr: NaiveTime,
    pub asr: NaiveTime,
    pub maghrib: NaiveTime,
    pub isha: NaiveTime,
    pub jummah: NaiveTime,
}

/// Mosque details with references to prayer_times records
#[cfg(feature = "ssr")]
#[derive(Debug, Serialize, Deserialize)]
pub struct MosqueDetails {
    pub id: RecordId,
    pub mosque: RecordId,
    pub admins: Vec<RecordId>,
    pub jamat_times: RecordId,  
    pub adhan_times: RecordId,  
}

/// Mosque details with prayer times fetched/inlined
/// Use this when querying with FETCH jamat_times, adhan_times
#[cfg(feature = "ssr")]
#[derive(Debug, Serialize, Deserialize)]
pub struct MosqueDetailsWithTimes {
    pub id: RecordId,
    pub mosque: RecordId,
    pub admins: Vec<RecordId>,
    pub jamat_times: PrayerTimes,
    pub adhan_times: PrayerTimes,
}

/// For creating new mosque details
#[cfg(feature = "ssr")]
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateMosqueDetails {
    pub mosque: RecordId,
    pub admins: Vec<RecordId>,
    pub jamat_times: RecordId,
    pub adhan_times: RecordId,
}