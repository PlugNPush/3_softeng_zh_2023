use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::TemperatureMeasurement;

const CAPACITY: usize = 100;

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MeasurementList {
    measurements: BTreeSet<TemperatureMeasurement>,
}

impl MeasurementList {
    pub fn insert(&mut self, measurement: TemperatureMeasurement) {
        self.measurements.insert(measurement);
        if self.measurements.len() > CAPACITY {
            self.measurements.pop_last();
        }
    }

    pub fn clear(&mut self) {
        self.measurements.clear();
    }
}

impl IntoIterator for MeasurementList {
    type Item = TemperatureMeasurement;
    type IntoIter = std::collections::btree_set::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.measurements.into_iter()
    }
}
