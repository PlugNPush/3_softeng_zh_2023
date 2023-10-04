use leptos::*;

use crate::components::{measurement_list::MeasurementList, TitleBar};

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="h-screen flex flex-col gap-2 flex-nowrap">

            <TitleBar />

            <div class="flex flex-col w-full items-center overflow-scroll
                        relative h-full">
                <MeasurementList />
            </div>
        </div>
    }
}
