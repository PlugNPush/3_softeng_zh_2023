use models::TemperatureMeasurement;

use super::State;

#[derive(Debug)]
pub enum Action {
    Overwrite(State),
    Insert(TemperatureMeasurement),
    Clear,
}
