use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

use crate::TemperatureMeasurement;

const CAPACITY: usize = 100;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MeasurementList {
    measurements: VecDeque<TemperatureMeasurement>,
}

impl Default for MeasurementList {
    fn default() -> Self {
        Self {
            measurements: VecDeque::with_capacity(CAPACITY),
        }
    }
}

impl MeasurementList {
    pub fn insert(&mut self, measurement: TemperatureMeasurement) {
        if self.measurements.len() >= CAPACITY {
            self.measurements.pop_back();
        }
        self.measurements.push_front(measurement);
    }

    pub fn clear(&mut self) {
        self.measurements.clear();
    }
}

impl IntoIterator for MeasurementList {
    type Item = TemperatureMeasurement;
    type IntoIter = std::collections::vec_deque::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.measurements.into_iter()
    }
}
