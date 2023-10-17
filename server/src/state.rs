use std::sync::Arc;

use models::{MeasurementList, Notification, TemperatureMeasurement};
use tokio::sync::{broadcast, Mutex};

#[derive(Debug, Clone)]
pub struct AppState {
    measurements: Arc<Mutex<MeasurementList>>,
    sender: Arc<broadcast::Sender<Notification>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            measurements: Default::default(),
            sender: Arc::new(broadcast::channel(100).0),
        }
    }
}

impl AppState {
    /// clones the measurements
    pub async fn get_measurements(&self) -> MeasurementList {
        self.measurements.lock().await.clone()
    }

    fn publish(&self, notification: Notification) {
        // don't care if nobody is listening
        self.sender.send(notification).ok();
    }

    pub async fn insert_measurement(&self, measurement: TemperatureMeasurement) {
        let mut guard = self.measurements.lock().await;
        guard.insert(measurement.clone());
        self.publish(Notification::New(measurement));
    }

    pub async fn delete_all(&self) {
        self.measurements.lock().await.clear();
        self.publish(Notification::Cleared);
    }

    pub fn subscribe(&self) -> broadcast::Receiver<Notification> {
        self.sender.subscribe()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn can_insert_and_get_measurements() {
        let state = AppState::default();
        let measurement = TemperatureMeasurement::random();
        state.insert_measurement(measurement.clone()).await;
        let measurements = state.get_measurements().await;

        let mut iter = measurements.into_iter();
        assert_eq!(iter.next(), Some(measurement));
        assert_eq!(iter.next(), None);
    }

    #[tokio::test]
    async fn notifications_are_published() {
        let state = AppState::default();
        let mut receiver = state.subscribe();
        let measurement = TemperatureMeasurement::random();
        state.insert_measurement(measurement.clone()).await;

        let notification = receiver.recv().await.unwrap();
        assert_eq!(notification, Notification::New(measurement));
    }
}
