use leptos::*;

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
        <div class="flex flex-col md:flex-row gap-2 p-2 w-full">
            <div class="hidden md:flex flex-1" />

            <Title />

            <div class="flex flex-1 gap-2 self-center md:justify-end">
                <ReloadButton />
                <InsertRandomButton />
                <DeleteAllButton />
                <DocumentationButton />
            </div>
        </div>
    }
}
