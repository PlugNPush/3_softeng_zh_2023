use leptos::*;

use super::{
    middleware::{self, PreMiddlewareAction},
    reducer::reduce,
    State,
};

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
        store.dispatch(PreMiddlewareAction::Reload);
        store
    }

    pub fn dispatch(self, action: PreMiddlewareAction) {
        spawn_local(async move {
            let action = middleware::process(action).await;
            let new_state = reduce(&self.state.get_untracked(), action);
            self.state.set(new_state);
        });
    }
}

pub fn provide_store() {
    provide_context(Store::new());
}

pub fn use_store() -> Store {
    use_context().expect("should find store context")
}

/// These impls are a little gnarly looking.
/// They are not necessary, but they make the store callable.
/// Essentially, it turns the [Store] into a custom closure type.
/// This is what leptos does for its own signal types, so I did it here too.
/// This is a nightly feature, activated by `#![feature(fn_traits)]`.
mod impl_fn_for_store {
    use leptos::SignalGet;

    use super::{State, Store};

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
}
