use leptos::*;

use super::Store;

pub fn provide_store() {
    provide_context(Store::new());
}

pub fn use_store() -> Store {
    use_context().expect("should find store context")
}
