use leptos::{component, view, IntoView};

use crate::{
    components::IconButton,
    icons::ReloadIcon,
    state::{use_store, Action},
};

#[component]
pub fn ReloadButton() -> impl IntoView {
    let store = use_store();

    view! {
        <IconButton on:click=move |_| store.dispatch(Action::Reload) aria_label="reload">
            <ReloadIcon/>
        </IconButton>
    }
}
