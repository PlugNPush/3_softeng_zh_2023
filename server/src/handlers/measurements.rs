use axum::{extract::State, Json};
use models::{TemperatureMeasurement, Id};

use crate::state::AppState;

// axum handler to get all tempature measurements
pub async fn get_all(state: State<AppState>) -> Json<Vec<TemperatureMeasurement>> {
    Json(state.get_measurements().await)
}

pub async fn insert(state: State<AppState>, Json(mut measurement): Json<TemperatureMeasurement>) {
    measurement.id = Id::random();
    state.insert_measurement(measurement).await;
}
