use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::id::Id;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemperatureMeasurement {
    pub id: Id<Self>,
    pub timestamp: DateTime<Utc>,
    // TODO document unit of measurement or use a dedicated type
    pub temperature: i64,
}

impl PartialEq<TemperatureMeasurement> for TemperatureMeasurement {
    fn eq(&self, other: &TemperatureMeasurement) -> bool {
        self.id == other.id
    }
}

impl Eq for TemperatureMeasurement {}

impl Ord for TemperatureMeasurement {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // newest measurements are first
        self.timestamp.cmp(&other.timestamp).reverse()
    }
}

impl PartialOrd for TemperatureMeasurement {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
