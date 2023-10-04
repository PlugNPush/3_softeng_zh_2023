use std::sync::Arc;

use models::{MeasurementList, TemperatureMeasurement};
use tokio::sync::Mutex;

#[derive(Debug, Clone, Default)]
pub struct AppState {
    measurements: Arc<Mutex<MeasurementList>>,
}

impl AppState {
    /// clones the measurements
    pub async fn get_measurements(&self) -> MeasurementList {
        self.measurements.lock().await.clone()
    }

    /// max capacity of 100 ([MAX_MEASUREMENTS]).
    /// If the queue is full, the oldest measurement is removed.
    pub async fn insert_measurement(&self, value: TemperatureMeasurement) {
        let mut guard = self.measurements.lock().await;
        guard.insert(value);
    }

    pub async fn delete_all(&self) {
        self.measurements.lock().await.clear();
    }
}
