use leptos::*;

#[component]
pub fn IconButton(aria_label: &'static str, children: Children) -> impl IntoView {
    view! {
        <button class="bg-slate-600 rounded-full p-2 w-min" aria-label=aria_label>
            { children() }
        </button>
    }
}
