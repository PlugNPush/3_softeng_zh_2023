use std::time::Duration;

use leptos::*;
use models::TemperatureMeasurement;

#[component]
pub fn Measurement(idx: usize, measurement: TemperatureMeasurement) -> impl IntoView {
    let should_transition = RwSignal::new(false);

    // Allow some time for the initial render.
    // The transition will only be triggered if the initial render is done.
    set_timeout(
        move || should_transition.set(true),
        Duration::from_millis(50),
    );

    view! {
        <div
            class="flex flex-col gap-2 absolute"
            style:top=move || format!("{}em", idx * 5)
            style:transition="transform 0.2s ease-in"
            style:transform=move || {
                if should_transition.get() {
                    "translateY(0)"
                } else {
                    "translateY(-5em)"
                }
            }
        >
            <div class="text-2xl">{ measurement.temperature }"°C"</div>
            <div class="text-sm">{ measurement.timestamp.to_string() }</div>
        </div>
    }
}
