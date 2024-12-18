use leptos::window;
use reqwest::Url;
use wasm_bindgen::UnwrapThrowExt;

/// Retrieves the current `Window` hostname.
///
/// # Panics
///
/// Panics if the hostname is not a valid URL, or if `window.location.hostname` is `None`.
pub fn hostname() -> Url {
    let url = &window().location().origin().unwrap_throw();
    Url::parse(url).unwrap_throw()
}
