//! In the redux pattern, reducers must be pure functions. This means that they
//! cannot perform side effects. This has several benefits, but it means that we
//! need to do some extra work to perform side effects. This is where middleware
//! comes in. Instead of dispatching an action directly, we process it by a
//! middleware first. The middleware can then perform side effects and dispatch
//! a new action. This new action is then passed to the reducer. Pre- and
//! post-middleware actions have different types, so there is no risk of
//! forgetting to run a middleware outside of the store module.

use gloo::net::http::Request;
use models::TemperatureMeasurement;

use super::{action::Action, State};

pub enum PreMiddlewareAction {
    Reload,
    InsertRandom,
    DeleteAll,
}

pub async fn process(action: PreMiddlewareAction) -> Action {
    match action {
        PreMiddlewareAction::Reload => {
            let state = Request::get("/api/measurements")
                .send()
                .await
                .unwrap()
                .json::<State>()
                .await
                .unwrap();
            Action::Overwrite(state)
        }
        PreMiddlewareAction::InsertRandom => {
            let measurement = Request::post("/api/measurements/random")
                .send()
                .await
                .unwrap()
                .json::<TemperatureMeasurement>()
                .await
                .unwrap();
            Action::Insert(measurement)
        }
        PreMiddlewareAction::DeleteAll => {
            Request::delete("/api/measurements").send().await.unwrap();
            Action::Clear
        }
    }
}
