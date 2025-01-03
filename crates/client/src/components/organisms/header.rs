use leptos::{component, view, IntoView};

use crate::components::molecules::navigation_bar::NavigationBar;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="flex flex-wrap md:justify-start md:flex-nowrap z-50 w-full ">
        <NavigationBar/>
    </header>
    }
}
