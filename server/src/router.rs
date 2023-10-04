use axum::{
    routing::{delete, get, post},
    Router,
};

use crate::{handlers::measurements as tm, state::AppState};

pub fn api_router() -> Router {
    Router::new()
        // handlers
        .route("/measurements", get(tm::get_all))
        .route("/measurements", post(tm::insert))
        .route("/measurements/random", post(tm::insert_random))
        .route("/measurements", delete(tm::delete_all))
        //
        // shared state
        .with_state(AppState::default())
}
