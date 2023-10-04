use leptos::{component, view, IntoView};

use crate::{
    components::IconButton,
    icons::PlusIcon,
    store::{use_store, Action},
};

#[component]
pub fn InsertRandomButton() -> impl IntoView {
    let store = use_store();

    view! {
        <IconButton on:click=move |_| store.dispatch(Action::InsertRandom) >
            <PlusIcon />
        </IconButton>
    }
}
