use leptos::*;
use models::TemperatureMeasurement;

use crate::{components::FlexSpace, icons::InfoCircleIcon};

#[cfg(debug_assertions)]
static DOCS_HREF: &str = "http://localhost:5000";
#[cfg(not(debug_assertions))]
static DOCS_HREF: &str = "/docs";

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    let measurements = create_resource_with_initial_value(
        cx,
        || (),
        |_| async {
            gloo::net::http::Request::get("/api/measurements")
                .send()
                .await
                .unwrap()
                .json::<Vec<TemperatureMeasurement>>()
                .await
                .unwrap()
        },
        Some(Vec::new()),
    );

    view! { cx,
        <div class="h-screen flex flex-col gap-2 flex-nowrap">

            // tab bar, kinda empty right now
            <div class="flex gap-2 p-2 w-full">
                <FlexSpace />

                <div class="text-2xl self-center">
                    "Temperature Measurements"
                </div>

                <FlexSpace />

                <a
                    class="bg-slate-600 rounded-full p-2 w-min absolute right-2 top-2"
                    href=DOCS_HREF
                    rel="external" // make sure leptos doesn't use client-side routing
                >
                    <InfoCircleIcon/>
                </a>
            </div>

            <div class="flex flex-col w-full items-center overflow-scroll">
                <For
                    each=move || measurements.read(cx).unwrap()
                    key=move |m| m.id.clone()
                    view=|cx, m| view! { cx,
                        <div class="flex gap-2 p-2">
                            <div class="flex flex-col gap-2">
                                <div class="text-2xl">{ m.temperature }"°C"</div>
                                <div class="text-sm">{ m.timestamp.to_string() }</div>
                            </div>
                        </div>
                    }
                />
            </div>
        </div>
    }
}
