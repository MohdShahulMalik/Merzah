#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GeoPosition {
    pub latitude: f64,
    pub longitude: f64,
}

impl GeoPosition {
    pub fn new(latitude: f64, longitude: f64) -> Self {
        Self {
            latitude,
            longitude,
        }
    }

    pub fn as_lat_lon(&self) -> (f64, f64) {
        (self.latitude, self.longitude)
    }
}
