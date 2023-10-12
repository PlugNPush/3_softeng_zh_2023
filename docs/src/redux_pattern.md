# Redux Pattern Implementation

Part of the requirements for this project is to use the Redux pattern on the frontend.
For popular frontend UI-libaries like React and Vue,
implementations of this pattern are readily available.
However, we are using the Rust-based UI-library Leptos.
Rust is a much less common language to write frontend code in,
so the ecosystem is not as developed in this domain.
While there are [crates claiming to implement the Redux pattern][redux-crates],
their download numbers do not spark confidence.
We therefore chose to implement the pattern ourselves.

Code snippets are all from files located in `app/src/state/`.
They are abbreviated for clarity where appropriate.

As Rust is a statically typed language,
we first define the structure of our global state.
It is simply an alias for the list of our measurements.

```rust
// mod.rs
type State = MeasurementList;
```

This state will be managed by the *store*,
whose first purpose is to allow UI components to subscribe to state changes.
Note that the `State` is wrapped in the type `RwSignal`,
which is Leptos' reactivity primitive.

```rust
// store.rs
pub struct Store {
    state: RwSignal<State>,
}
```

An essential aspect of the Redux pattern is the *reducer*.
It is a function that takes the current state and an action,
and returns the new state.
The actions and the reducer go hand-in-hand to enforce
well-defined state transitions throughout the application.
Rust's enums and exhaustive pattern matching enable us to do this elegantly
and in a robust, type-safe manner.

```rust
// action.rs
pub enum Action {
    Overwrite(State),
    Insert(TemperatureMeasurement),
    Clear,
}

// reducer.rs
pub fn reduce(state: &State, action: Action) -> State {
    match action {
        Action::Overwrite(state) => state,
        Action::Insert(measurement) => // ...
        Action::Clear => State::default(),
    }
}
```

An important rule of the Redux pattern is that the reducer must be a pure function.
This means that it must not have any side effects,
like making API requests to our server.
But these API requests are necessary, as our frontend store is supposed
to reflect the server-side state.
For this purpose, the Redux pattern introduces the concept of *middleware*.
A middleware may intercept actions before they reach the reducer
and perform arbitrary side effects, even modify the action itself.
So, while actions dispatched by our frontend mostly express
*intentions to mutate server-side state*, these actions will be intercepted
by the middleware, which makes API requests and replaces the received actions
with ones that express *actually changed* server-side state.

```rust
// middleware.rs
pub enum PreMwAction {
    Reload,
    InsertRandom,
    DeleteAll,
}
pub async fn process(action: PreMwAction) -> Action {
    match action {
        PreMwAction::Reload => Request::get("/api/measurements") // ...
        PreMwAction::InsertRandom => Request::post("/api/measurements/random") // ...
        PreMwAction::DeleteAll => Request::delete("/api/measurements") // ...
    }
}
```

To tie it all together, we need a function that dispatches actions
on the store. Since we defined separate types for actions that have
been processed by the middleware and those that have not,
Rust's type system will prevent us from dispatching the wrong type of action
or forgetting to process an action using the middleware in the right place.

```rust
// store.rs
impl Store {
    pub fn dispatch(self, action: PreMwAction) {
        spawn_local(async move {
            let action = middleware::process(action).await;
            self.dispatch_without_middleware(action);
        });
    }
    fn dispatch_without_middleware(&self, action: Action) {
        let new_state = reduce(&self.state.get_untracked(), action);
        if new_state != self.state.get_untracked() {
            self.state.set(new_state);
        }
    }
}
```

While this is maybe the simplest possible implementation of the pattern,
it is fully functional, ergonomic and robust.

[redux-crates]: https://crates.io/search?q=redux
