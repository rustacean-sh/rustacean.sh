use leptos::{component, view, IntoView};

use crate::components::atoms::github_stars::GitHubStars;

#[component]

pub fn NavigationBar() -> impl IntoView {
    view! {
        <nav class="relative max-w-7xl w-full flex flex-wrap  basis-full items-center px-4 md:px-6 md:px-8 mx-auto">
                    <div class="md:col-span-3">
                        <a class="flex-none rounded-xl text-xl inline-block font-semibold focus:outline-none focus:opacity-80" href="#" aria-label="Preline">
                            <figure class=" rounded-full overflow-hidden">
                                <img  height="110" width="110" src={"../assets/images/rustacean-flat-happy.svg"} alt={"Ferris, the crab"} />
                            </figure>
                        </a>
                    </div>
                     <GitHubStars />

                </nav>
    }
}
