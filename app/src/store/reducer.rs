use models::TemperatureMeasurement;

use super::{action::Action, State};

async fn reload() -> State {
    gloo::net::http::Request::get("/api/measurements")
        .send()
        .await
        .unwrap()
        .json::<State>()
        .await
        .unwrap()
}

pub async fn reduce(state: &State, action: Action) -> State {
    match action {
        Action::Reload => reload().await,
        Action::InsertRandom => {
            let measurement = gloo::net::http::Request::post("/api/measurements/random")
                .send()
                .await
                .unwrap()
                .json::<TemperatureMeasurement>()
                .await
                .unwrap();
            let mut new_state = state.clone();
            new_state.insert(measurement);
            new_state
        }
        Action::DeleteAll => {
            gloo::net::http::Request::delete("/api/measurements")
                .send()
                .await
                .unwrap();
            Default::default()
        }
    }
}
