use super::{action::Action, State};

pub fn reduce(state: &State, action: Action) -> State {
    match action {
        Action::Overwrite(state) => state,
        Action::Insert(measurement) => {
            let mut new_state = state.clone();
            new_state.insert(measurement);
            new_state
        }
        Action::Clear => State::default(),
    }
}
