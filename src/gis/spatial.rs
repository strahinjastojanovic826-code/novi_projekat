#[derive(Debug, Clone, PartialEq)]
pub struct GeoPoint {
    pub lat: f64, // Geografska širina (-90.0 do 90.0)
    pub lon: f64, // Geografska dužina (-180.0 do 180.0)
}

impl GeoPoint {
    pub fn new(lat: f64, lon: f64) -> Self {
        Self { lat, lon }
    }

    /// Izračunavanje Haversine udaljenosti između dve tačke na Zemlji u kilometrima
    pub fn distance_to_km(&self, other: &GeoPoint) -> f64 {
        let earth_radius_km = 6371.0;

        let d_lat = (other.lat - self.lat).to_radians();
        let d_lon = (other.lon - self.lon).to_radians();

        let lat1 = self.lat.to_radians();
        let lat2 = other.lat.to_radians();

        let a = (d_lat / 2.0).sin().powi(2)
            + lat1.cos() * lat2.cos() * (d_lon / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

        earth_radius_km * c
    }
}

#[derive(Debug, Clone)]
pub struct BoundingBox {
    pub min_lat: f64,
    pub max_lat: f64,
    pub min_lon: f64,
    pub max_lon: f64,
}

impl BoundingBox {
    pub fn new(min_lat: f64, max_lat: f64, min_lon: f64, max_lon: f64) -> Self {
        Self {
            min_lat,
            max_lat,
            min_lon,
            max_lon,
        }
    }

    pub fn contains(&self, point: &GeoPoint) -> bool {
        point.lat >= self.min_lat
            && point.lat <= self.max_lat
            && point.lon >= self.min_lon
            && point.lon <= self.max_lon
    }
}

#[derive(Debug, Clone)]
pub struct SpatialFeature {
    pub id: u32,
    pub name: String,
    pub category: String, // npr. "DataCenter", "Tower", "User", "City"
    pub location: GeoPoint,
}