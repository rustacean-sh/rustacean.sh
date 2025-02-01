pub mod components;
pub mod pages;
pub mod services;
pub mod utils;

use leptos::{component, view, IntoView};
use leptos_meta::provide_meta_context;
use leptos_router::{Route, Router, Routes};

use self::components::templates::website::Website;
use self::pages::About;
use self::pages::Members;
use self::pages::NotFound;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Router>
            <Routes>
                <Route path="/" view=Website>
                    <Route path="/" view=Members />
                    <Route path="/about" view=About />
                </Route>
                <Route path="/*" view=NotFound />
            </Routes>
        </Router>
    }
}
