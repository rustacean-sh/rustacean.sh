use leptos::{component, view, IntoView};

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <div id="not-found">
            <h1>"Are you lost?"</h1>
        </div>
    }
}
