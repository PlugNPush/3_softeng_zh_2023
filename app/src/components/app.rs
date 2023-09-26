use leptos::*;
use leptos_router::*;

use crate::components::Home;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <Router>
            <Providers>
                <AppRoutes/>
            </Providers>
        </Router>
    }
}

#[component]
pub fn Providers(cx: Scope, children: Children) -> impl IntoView {
    // add providers here if needed

    children(cx)
}

#[component]
pub fn AppRoutes(cx: Scope) -> impl IntoView {
    view! { cx,
        <Routes>
            <Route path="/" view=|cx| view! { cx, <Home/> }/>
            // add more routes here if needed
        </Routes>
    }
}
