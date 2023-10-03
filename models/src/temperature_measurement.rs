use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::id::Id;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TemperatureMeasurement {
    pub id: Id<Self>,
    pub timestamp: DateTime<Utc>,
    // TODO document unit of measurement or use a dedicated type
    pub temperature: i64,
}
