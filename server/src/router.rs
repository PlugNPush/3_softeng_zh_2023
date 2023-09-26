use axum::{
    routing::{get, post},
    Router,
};

use crate::{handlers::measurements as tm, state::AppState};

pub fn api_router() -> Router {
    Router::new()
        // handlers
        .route("/measurements", get(tm::get_all))
        .route("/measurements", post(tm::insert))
        //
        // shared state
        .with_state(AppState::default())
}
