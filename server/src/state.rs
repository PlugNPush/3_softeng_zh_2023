use std::{collections::VecDeque, sync::Arc};

use models::TemperatureMeasurement;
use tokio::sync::Mutex;

const MAX_MEASUREMENTS: usize = 100;

#[derive(Debug, Clone)]
pub struct AppState {
    measurements: Arc<Mutex<VecDeque<TemperatureMeasurement>>>,
}

impl AppState {
    /// clones the measurements
    pub async fn get_measurements(&self) -> Vec<TemperatureMeasurement> {
        self.measurements.lock().await.clone().into()
    }

    /// max capacity of 100 ([MAX_MEASUREMENTS]).
    /// If the queue is full, the oldest measurement is removed.
    pub async fn insert_measurement(&self, value: TemperatureMeasurement) {
        let mut guard = self.measurements.lock().await;
        if guard.len() >= MAX_MEASUREMENTS {
            guard.pop_front();
        }
        guard.push_back(value);
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            measurements: Arc::new(Mutex::new(VecDeque::with_capacity(MAX_MEASUREMENTS))),
        }
    }
}
