use serde::{Deserialize, Serialize};

use crate::TemperatureMeasurement;

/// Data type used for server-to-client communication via websocket.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Notification {
    New(TemperatureMeasurement),
    Cleared,
}
