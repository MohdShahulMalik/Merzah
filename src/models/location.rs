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

    pub fn distance_km(&self, other: &GeoPosition) -> f64 {
        const EARTH_RADIUS_KM: f64 = 6371.0;
        let d_lat = (other.latitude - self.latitude).to_radians();
        let d_lon = (other.longitude - self.longitude).to_radians();
        let lat1 = self.latitude.to_radians();
        let lat2 = other.latitude.to_radians();

        let a = (d_lat / 2.0).sin().powi(2)
            + lat1.cos() * lat2.cos() * (d_lon / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
        EARTH_RADIUS_KM * c
    }
}
