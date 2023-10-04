use leptos::{component, view, IntoView};

use crate::{
    components::IconButton,
    icons::ReloadIcon,
    store::{use_store, Action},
};

#[component]
pub fn ReloadButton() -> impl IntoView {
    let store = use_store();

    view! {
        <IconButton on:click=move |_| store.dispatch(Action::Reload) >
            <ReloadIcon/>
        </IconButton>
    }
}
