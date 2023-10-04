use leptos::*;

use crate::store::use_store;

mod measurement;

use measurement::Measurement;

#[component]
pub fn MeasurementList() -> impl IntoView {
    let store = use_store();

    view! {
        {
            move || store().into_iter().enumerate().map(|m| {
                view! {
                    <Measurement idx=m.0 measurement=m.1 />
                }
            })
            .collect_view()
        }
    }
}
