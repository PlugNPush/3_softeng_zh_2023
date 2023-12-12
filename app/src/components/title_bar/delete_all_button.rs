use leptos::{component, view, IntoView};

use crate::{
    components::IconButton,
    icons::TrashIcon,
    state::{use_store, Action},
};

#[component]
pub fn DeleteAllButton() -> impl IntoView {
    let store = use_store();

    view! {
        <IconButton on:click=move |_| store.dispatch(Action::DeleteAll) aria_label="delete">
            <TrashIcon />
        </IconButton>
    }
}
