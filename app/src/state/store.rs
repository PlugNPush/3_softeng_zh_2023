use leptos::*;
use leptos_use::{use_websocket, UseWebsocketReturn};
use models::Notification;

use super::{
    action::Action,
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

        let UseWebsocketReturn { message, .. } = use_websocket("/api/notifications");
        create_effect(move |_| {
            if let Some(message) = message.get() {
                let notification = serde_json::from_str::<Notification>(&message).unwrap();
                let action = match notification {
                    Notification::New(measurements) => Action::Insert(measurements),
                    Notification::Cleared => Action::Clear,
                };
                store.dispatch_without_middleware(action);
            }
        });

        store
    }

    fn dispatch_without_middleware(&self, action: Action) {
        let new_state = reduce(&self.state.get_untracked(), action);
        if new_state != self.state.get_untracked() {
            self.state.set(new_state);
        }
    }

    pub fn dispatch(self, action: PreMiddlewareAction) {
        spawn_local(async move {
            let action = middleware::process(action).await;
            self.dispatch_without_middleware(action);
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
