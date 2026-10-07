use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct NearbyMosqueCardItem {
    pub id: String,
    pub mosque_name: String,
    pub iqamah_label: String,
    pub iqamah_time: String,
    pub distance: f64,
    pub is_favorite: bool,
    pub image_url: Option<String>,
}

impl NearbyMosqueCardItem {
    pub fn new(
        id: String,
        mosque_name: String,
        iqamah_label: String,
        iqamah_time: String,
        distance: f64,
        is_favorite: bool,
        image_url: Option<String>,
    ) -> Self {
        Self {
            id,
            mosque_name,
            iqamah_label,
            iqamah_time,
            distance,
            is_favorite,
            image_url,
        }
    }
}
