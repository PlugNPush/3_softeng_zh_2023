use models::MeasurementList;

mod action;
mod middleware;
mod reducer;
mod store;

use self::middleware::PreMiddlewareAction;
pub use self::store::{provide_store, use_store};

/// The type of data managed by the store.
type State = MeasurementList;

// Users of the store only need to know about the pre-middleware actions.
// The fact that middleware is run is an implementation detail.
pub type Action = PreMiddlewareAction;
