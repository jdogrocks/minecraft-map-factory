use serde::{Deserialize, Serialize};
use std::path::Path;

/// A geographic location for map generation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    /// Human-readable name.
    pub name: String,

    /// US state or region.
    pub state: String,

    /// Bounding box: min_lat, min_lng, max_lat, max_lng.
    pub bbox: [f64; 4],

    /// Expected complexity tier (small, medium, large).
    #[serde(default = "default_tier")]
    pub tier: String,

    /// Tags for categorization.
    #[serde(default)]
    pub tags: Vec<String>,
}

/// Tracks generation status for a location.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LocationStatus {
    Pending,
    InProgress,
    Completed,
    Failed { attempts: u32, last_error: String },
    Skipped,
}

/// Location database loaded from TOML.
#[derive(Debug)]
pub struct LocationDatabase {
    locations: Vec<Location>,
    statuses: Vec<LocationStatus>,
}

#[derive(Debug, Deserialize)]
struct LocationFile {
    #[serde(rename = "location")]
    locations: Vec<Location>,
}

impl LocationDatabase {
    pub fn from_file(path: &Path) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let content = std::fs::read_to_string(path)?;
        let file: LocationFile = toml::from_str(&content)?;
        let count = file.locations.len();
        Ok(Self {
            locations: file.locations,
            statuses: vec![LocationStatus::Pending; count],
        })
    }

    /// Returns the next pending location, prioritizing smaller tiers first.
    pub fn next_pending(&self) -> Option<(usize, &Location)> {
        let tier_order = |t: &str| -> u8 {
            match t {
                "small" => 0,
                "medium" => 1,
                "large" => 2,
                _ => 3,
            }
        };

        let mut candidates: Vec<(usize, &Location)> = self
            .locations
            .iter()
            .enumerate()
            .filter(|(i, _)| self.statuses[*i] == LocationStatus::Pending)
            .collect();

        candidates.sort_by_key(|(_, loc)| tier_order(&loc.tier));
        candidates.into_iter().next()
    }

    /// Returns count of locations in each status.
    pub fn status_summary(&self) -> StatusSummary {
        let mut summary = StatusSummary::default();
        for status in &self.statuses {
            match status {
                LocationStatus::Pending => summary.pending += 1,
                LocationStatus::InProgress => summary.in_progress += 1,
                LocationStatus::Completed => summary.completed += 1,
                LocationStatus::Failed { .. } => summary.failed += 1,
                LocationStatus::Skipped => summary.skipped += 1,
            }
        }
        summary
    }

    pub fn set_status(&mut self, index: usize, status: LocationStatus) {
        if index < self.statuses.len() {
            self.statuses[index] = status;
        }
    }

    pub fn get_location(&self, index: usize) -> Option<&Location> {
        self.locations.get(index)
    }

    pub fn total(&self) -> usize {
        self.locations.len()
    }

    /// Returns the bbox string formatted for the map generator CLI.
    pub fn bbox_string(location: &Location) -> String {
        format!(
            "{},{},{},{}",
            location.bbox[0], location.bbox[1], location.bbox[2], location.bbox[3]
        )
    }

    /// Shrink a location's bbox by a factor (used for retry with reduced area).
    pub fn shrink_bbox(location: &Location, factor: f64) -> [f64; 4] {
        let center_lat = (location.bbox[0] + location.bbox[2]) / 2.0;
        let center_lng = (location.bbox[1] + location.bbox[3]) / 2.0;
        let half_lat = (location.bbox[2] - location.bbox[0]) / 2.0 * factor;
        let half_lng = (location.bbox[3] - location.bbox[1]) / 2.0 * factor;
        [
            center_lat - half_lat,
            center_lng - half_lng,
            center_lat + half_lat,
            center_lng + half_lng,
        ]
    }
}

#[derive(Debug, Default)]
pub struct StatusSummary {
    pub pending: usize,
    pub in_progress: usize,
    pub completed: usize,
    pub failed: usize,
    pub skipped: usize,
}

fn default_tier() -> String {
    "medium".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_locations() -> LocationDatabase {
        LocationDatabase {
            locations: vec![
                Location {
                    name: "Test Small".into(),
                    state: "CA".into(),
                    bbox: [34.0, -118.3, 34.01, -118.29],
                    tier: "small".into(),
                    tags: vec![],
                },
                Location {
                    name: "Test Large".into(),
                    state: "NY".into(),
                    bbox: [40.7, -74.0, 40.8, -73.9],
                    tier: "large".into(),
                    tags: vec![],
                },
                Location {
                    name: "Test Medium".into(),
                    state: "TX".into(),
                    bbox: [29.7, -95.4, 29.75, -95.35],
                    tier: "medium".into(),
                    tags: vec![],
                },
            ],
            statuses: vec![
                LocationStatus::Pending,
                LocationStatus::Pending,
                LocationStatus::Pending,
            ],
        }
    }

    #[test]
    fn test_next_pending_prioritizes_small() {
        let db = sample_locations();
        let (idx, loc) = db.next_pending().unwrap();
        assert_eq!(idx, 0);
        assert_eq!(loc.tier, "small");
    }

    #[test]
    fn test_status_tracking() {
        let mut db = sample_locations();
        db.set_status(0, LocationStatus::Completed);
        db.set_status(
            1,
            LocationStatus::Failed {
                attempts: 3,
                last_error: "OOM".into(),
            },
        );

        let summary = db.status_summary();
        assert_eq!(summary.completed, 1);
        assert_eq!(summary.failed, 1);
        assert_eq!(summary.pending, 1);

        let (idx, loc) = db.next_pending().unwrap();
        assert_eq!(idx, 2);
        assert_eq!(loc.tier, "medium");
    }

    #[test]
    fn test_bbox_string() {
        let loc = Location {
            name: "Test".into(),
            state: "CA".into(),
            bbox: [34.0, -118.3, 34.01, -118.29],
            tier: "small".into(),
            tags: vec![],
        };
        assert_eq!(
            LocationDatabase::bbox_string(&loc),
            "34,-118.3,34.01,-118.29"
        );
    }

    #[test]
    fn test_shrink_bbox() {
        let loc = Location {
            name: "Test".into(),
            state: "CA".into(),
            bbox: [0.0, 0.0, 10.0, 10.0],
            tier: "small".into(),
            tags: vec![],
        };
        let shrunk = LocationDatabase::shrink_bbox(&loc, 0.5);
        assert!((shrunk[0] - 2.5).abs() < 0.001);
        assert!((shrunk[2] - 7.5).abs() < 0.001);
    }
}
