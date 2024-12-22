pub mod components;
pub mod services;
pub mod utils;

use std::collections::BTreeMap;

use leptos::{
    component, create_memo, create_rw_signal, spawn_local, view, For, IntoView, Show, SignalGet,
    SignalSet,
};
use leptos_meta::provide_meta_context;
use leptos_router::Router;

use proto::rustacean::Rustacean;

use self::components::atoms::github_stars::GitHubStars;
use self::services::Services;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let rustaceans = create_rw_signal::<BTreeMap<String, Rustacean>>(BTreeMap::default());
    let rustaceans_list = create_memo(move |_| {
        rustaceans
            .get()
            .values()
            .map(|r| r.to_owned())
            .collect::<Vec<Rustacean>>()
    });
    let error = create_rw_signal::<Option<String>>(None);

    spawn_local(async move {
        let services = Services::new();

        match services.api().list_rustaceans().await {
            Ok(db) => {
                rustaceans.set(db);
            }
            Err(err) => {
                error.set(Some(err.to_string()));
            }
        }
    });

    view! {
        <Router>
            <header class="flex flex-wrap md:justify-start md:flex-nowrap z-50 w-full ">
                <nav class="relative max-w-7xl w-full flex flex-wrap md:grid md:grid-cols-12 basis-full items-center px-4 md:px-6 md:px-8 mx-auto">
                    <div class="md:col-span-3">
                        //   <!-- Logo -->
                        <a class="flex-none rounded-xl text-xl inline-block font-semibold focus:outline-none focus:opacity-80" href="#" aria-label="Preline">
                            <figure class=" rounded-full overflow-hidden">
                                <img  height="110" width="110" src={"../assets/images/rustacean-flat-happy.svg"} alt={"Ferris, the crab"} />
                            </figure>
                        </a>
                        //   <!-- End Logo -->
                    </div>
                    // <!-- Button Group -->
                    <GitHubStars />
                    // <!-- End Button Group -->
                    // <!-- Collapse -->
                    <div id="hs-navbar-hcail" class="hs-collapse hidden overflow-hidden transition-all duration-300 basis-full grow md:block md:w-auto md:basis-auto md:order-2 md:col-span-6" aria-labelledby="hs-navbar-hcail-collapse">
                        <div class="flex flex-col gap-y-4 gap-x-0 mt-5 md:flex-row md:justify-center md:items-center md:gap-y-0 md:gap-x-7 md:mt-0">
                            <div class="md:hidden">
                                <button type="button" class=" hs-collapse-toggle size-[38px] flex justify-center items-center text-sm font-semibold rounded-xl border border-black text-black hover:bg-gray-100 focus:outline-none focus:bg-gray-100 disabled:opacity-50 disabled:pointer-events-none" id="hs-navbar-hcail-collapse" aria-expanded="false" aria-controls="hs-navbar-hcail" aria-label="Toggle navigation" data-hs-collapse="#hs-navbar-hcail">
                                    <svg class="hs-collapse-open:hidden shrink-0 size-4" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="3" x2="21" y1="6" y2="6"/><line x1="3" x2="21" y1="12" y2="12"/><line x1="3" x2="21" y1="18" y2="18"/></svg>
                                    <svg class="hs-collapse-open:block hidden shrink-0 size-4" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M18 6 6 18"/><path d="m6 6 12 12"/></svg>
                                </button>
                            </div>
                          //-----------------------//
                            <div>
                                <a class="inline-block text-black hover:text-gray-600 focus:outline-none focus:text-gray-600" href="#">About</a>
                            </div>
                            <div>
                                <a class="inline-block text-black hover:text-gray-600 focus:outline-none focus:text-gray-600" href="#">Code of Conduct</a>
                            </div>
                            <div>
                                <a class="inline-block text-black hover:text-gray-600 focus:outline-none focus:text-gray-600" href="#">Blog</a>
                            </div>
                        </div>
                    </div>
                    // <!-- End Collapse -->
                </nav>
            </header>
            <main class="flex flex-col justify-start items-center bg-gray-800 text-white h-screen font-bold">
                <h1 class="text-2xl py-4">"rustacean.sh"</h1>
                <p>"The Rustacean Hub, for contributors, projects and news."</p>
                <Show when=move || error.get().is_none() fallback=move || view! {
                    <span class="bg-rose-600 text-white p-4 rounded-md">
                        {error.get().unwrap_or_default()}
                    </span>
                }>
                    <ul class="py-4 space-y-4 w-10/12 md:w-[300px] mx-auto">
                        <For
                            key=|rus: &Rustacean| rus.name.clone()
                            each=move || rustaceans_list.get()
                            children=move |rus: Rustacean| {
                                view! {
                                    <li class="grid grid-rows-[70px,auto] gap-4">
                                        <figure class="h-[70px] w-[70px] rounded-full overflow-hidden">
                                            <img height="70" width="70" src={rus.image} alt={rus.name.clone()} />
                                        </figure>
                                        <article class="flex flex-col items-start justify-center">
                                            <strong>{rus.name}</strong>
                                            <a class="font-medium text-sm underline" target="blank" href={rus.gh_user}>
                                                "GitHub"
                                            </a>
                                        </article>
                                    </li>
                                }
                            }
                        />
                    </ul>
                </Show>
            </main>
            <footer class="mt-auto bg-gray-900 w-full dark:bg-neutral-950">
                <div class="mt-auto w-full max-w-[85rem] py-10 px-4 sm:px-6 lg:px-8 lg:pt-20 mx-auto">
                    <p class="text-center text-sm leading-6 text-white">
                        "lets think about a footer"
                    </p>
                </div>
            </footer>
        </Router>
    }
}
