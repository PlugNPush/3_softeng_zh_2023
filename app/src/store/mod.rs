use leptos::{spawn_local, RwSignal, SignalGet, SignalGetUntracked, SignalSet};
use models::MeasurementList;

mod action;
mod provider;
mod reducer;

pub use action::Action;
pub use provider::{provide_store, use_store};
use reducer::reduce;

type State = MeasurementList;

#[derive(Debug, Clone, Copy)]
pub struct Store {
    state: RwSignal<State>,
}

/// Basically a wrapper around [leptos::RwSignal]. The only difference is that
/// updates have to go through the `dispatch` method.
impl Store {
    fn new() -> Self {
        let store = Self {
            state: RwSignal::new(Default::default()),
        };
        // Load initial state
        store.dispatch(Action::Reload);
        store
    }

    pub fn dispatch(self, action: action::Action) {
        spawn_local(async move {
            let new_state = reduce(&self.state.get_untracked(), action).await;
            self.state.set(new_state);
        });
    }
}

impl FnOnce<()> for Store {
    type Output = State;

    extern "rust-call" fn call_once(self, _: ()) -> Self::Output {
        self.state.get()
    }
}

impl FnMut<()> for Store {
    extern "rust-call" fn call_mut(&mut self, _: ()) -> Self::Output {
        self.state.get()
    }
}

impl Fn<()> for Store {
    extern "rust-call" fn call(&self, _: ()) -> Self::Output {
        self.state.get()
    }
}
