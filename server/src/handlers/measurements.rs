use axum::{extract::State, Json};
use models::{Id, MeasurementList, TemperatureMeasurement};

use crate::state::AppState;

// axum handler to get all tempature measurements
pub async fn get_all(state: State<AppState>) -> Json<MeasurementList> {
    Json(state.get_measurements().await)
}

pub async fn insert(state: State<AppState>, Json(mut measurement): Json<TemperatureMeasurement>) {
    measurement.id = Id::random();
    state.insert_measurement(measurement).await;
}

pub async fn insert_random(state: State<AppState>) -> Json<TemperatureMeasurement> {
    let measurement = TemperatureMeasurement::random();
    state.insert_measurement(measurement.clone()).await;
    Json(measurement)
}

pub async fn delete_all(state: State<AppState>) {
    state.delete_all().await;
}
