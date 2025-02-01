use leptos::{component, view, IntoView};
use leptos_router::A;

use crate::components::atoms::github_stars::GitHubStars;

#[component]

pub fn NavigationBar() -> impl IntoView {
    view! {
        <nav class="w-full">
            <div class="safe-zone flex justify-between">
                <div class="">
                    <a id="logo" class="font-mono" href="#" aria-label="Preline">
                        <h1 class="text-rhs-orange-200">"rustacean.sh"</h1>
                    </a>
                </div>
                <div class="flex items-center gap-2">
                    <GitHubStars />
                    <a class="rsh-btn rsh-btn-primary" href="https://github.com/rustacean-sh/rustacean.sh" target="_blank">
                        "Join us"
                    </a>
                </div>
            </div>
            <div id="global-nav" class="font-mono flex items-center justify-center gap-x-4 py-2 px-4 md:px-2 mx-auto w-full md:max-w-[1200px] text-xs uppercase">
                <A href="/">"Members"</A>
                <A href="/about">"About"</A>
            </div>
        </nav>
    }
}
