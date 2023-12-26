use models::{Id, TemperatureMeasurement};
use rumqttc::{AsyncClient, Incoming, MqttOptions, QoS};
use std::{sync::Arc, time::Duration};
use tracing::debug;

use crate::state::AppState;

/// Subscribe to all devices in the temps topic on the confgured MQTT broker.
/// When receiving a new measurement, convert its first four bytes to float32
/// (le byte order) and insert it into the application state.
pub async fn subscribe(mqtt_url: &str, mqtt_port: u16, state: Arc<AppState>) {
    let mut options = MqttOptions::new("rumqtt-async", mqtt_url, mqtt_port);
    options.set_keep_alive(Duration::from_secs(5));

    let (client, mut eventloop) = AsyncClient::new(options, 10);
    let _ = client.subscribe("temps/#", QoS::AtLeastOnce).await;

    while let Ok(notification) = eventloop.poll().await {
        if let rumqttc::Event::Incoming(Incoming::Publish(publish)) = notification {
            let payload = publish.payload.to_vec();
            let payload: [u8; 4] = [payload[0], payload[1], payload[2], payload[3]];
            let temperature = f32::from_le_bytes(payload);
            let measurement = TemperatureMeasurement {
                id: Id::random(),
                timestamp: chrono::Utc::now(),
                temperature: temperature as i64,
            };

            state.insert_measurement(measurement).await;
            debug!(
                "Received temperature from {}: {}",
                publish.topic, temperature
            );
        }
    }
}
