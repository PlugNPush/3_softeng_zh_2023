use leptos::*;
use leptos_router::*;

use crate::components::Home;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Providers>
                <AppRoutes/>
            </Providers>
        </Router>
    }
}

#[component]
pub fn Providers(children: Children) -> impl IntoView {
    // add providers here if needed

    children()
}

#[component]
pub fn AppRoutes() -> impl IntoView {
    view! {
        <Routes>
            <Route path="/" view=Home />
            // add more routes here if needed
        </Routes>
    }
}
