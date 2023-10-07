use models::TemperatureMeasurement;

use super::State;

pub enum Action {
    Overwrite(State),
    Insert(TemperatureMeasurement),
    Clear,
}
