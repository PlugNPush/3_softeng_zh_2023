use leptos::*;

use crate::components::FlexSpace;

mod delete_all_button;
mod documentation_button;
mod insert_random_button;
mod reload_button;
mod title;

use delete_all_button::DeleteAllButton;
use documentation_button::DocumentationButton;
use insert_random_button::InsertRandomButton;
use reload_button::ReloadButton;
use title::Title;

#[component]
pub fn TitleBar() -> impl IntoView {
    view! {
        <div class="flex gap-2 p-2 w-full">
            <FlexSpace />

            <Title />

            <div class="flex-grow flex-1 flex gap-2 justify-end">
                <ReloadButton />
                <InsertRandomButton />
                <DeleteAllButton />
                <DocumentationButton />
            </div>
        </div>
    }
}
