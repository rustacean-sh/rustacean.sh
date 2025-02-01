use leptos::{component, view, IntoView};
use leptos_router::Outlet;

use crate::components::organisms::header::Header;

#[component]
pub fn Website() -> impl IntoView {
    view! {
        <div id="website">
            <Header/>
            <main>
                <Outlet />
            </main>
        </div>
    }
}
