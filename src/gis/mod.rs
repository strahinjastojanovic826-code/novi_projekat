pub mod spatial;

use spatial::{BoundingBox, GeoPoint, SpatialFeature};

pub struct QuantumSpatialEngine {
    pub features: Vec<SpatialFeature>,
    pub logs: Vec<String>,
    next_id: u32,
}

impl QuantumSpatialEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            features: Vec::new(),
            logs: Vec::new(),
            next_id: 1,
        };

        engine.logs.push("Quantum GIS Spatial Engine Inicijalizovan.".into());
        engine.seed_demo_locations();
        engine
    }

    pub fn add_feature(&mut self, name: &str, category: &str, lat: f64, lon: f64) -> u32 {
        let id = self.next_id;
        self.next_id += 1;

        let feature = SpatialFeature {
            id,
            name: name.to_string(),
            category: category.to_string(),
            location: GeoPoint::new(lat, lon),
        };

        self.features.push(feature);
        self.logs.push(format!("Dodat GIS objekat #{} '{}' ({:.4}, {:.4})", id, name, lat, lon));
        id
    }

    pub fn seed_demo_locations(&mut self) {
        // Demostracioni gradovi / data-centri
        self.add_feature("Beograd HQ DataCenter", "DataCenter", 44.7866, 20.4489);
        self.add_feature("Novi Sad Relay", "Tower", 45.2671, 19.8335);
        self.add_feature("Kragujevac Core Server", "DataCenter", 44.0128, 20.9114);
        self.add_feature("Niš Node", "Tower", 43.3209, 21.8958);
        self.add_feature("Subotica Sensor", "Sensor", 46.1005, 19.6650);
    }

    /// Radijus pretraga: Pronađi sve entitete u krugu od `radius_km` kilometara
    pub fn search_in_radius(&self, center: &GeoPoint, radius_km: f64) -> Vec<(&SpatialFeature, f64)> {
        let mut results = Vec::new();

        for feature in &self.features {
            let dist = center.distance_to_km(&feature.location);
            if dist <= radius_km {
                results.push((feature, dist));
            }
        }

        results.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        results
    }

    /// Bounding Box pretraga (BBOX)
    pub fn search_in_bbox(&self, bbox: &BoundingBox) -> Vec<&SpatialFeature> {
        self.features
            .iter()
            .filter(|f| bbox.contains(&f.location))
            .collect()
    }

    /// Pronađi K najbližih komšija (k-Nearest Neighbors - kNN)
    pub fn find_nearest(&self, center: &GeoPoint, k: usize) -> Vec<(&SpatialFeature, f64)> {
        let mut all_with_dist: Vec<(&SpatialFeature, f64)> = self
            .features
            .iter()
            .map(|f| (f, center.distance_to_km(&f.location)))
            .collect();

        all_with_dist.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        all_with_dist.into_iter().take(k).collect()
    }
}