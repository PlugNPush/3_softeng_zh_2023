use leptos::*;

use crate::icons::InfoCircleIcon;

#[cfg(debug_assertions)]
static DOCS_HREF: &str = "http://localhost:5000";
#[cfg(not(debug_assertions))]
static DOCS_HREF: &str = "/docs";

#[component]
pub fn DocumentationButton() -> impl IntoView {
    view! {
        <div class="bg-slate-600 rounded-full p-2 w-min">
            <a
                href=DOCS_HREF
                rel="external" // make sure leptos doesn't use client-side routing
            >
                <InfoCircleIcon/>
            </a>
        </div>
    }
}
