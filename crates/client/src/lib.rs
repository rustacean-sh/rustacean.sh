pub mod components;
pub mod services;
pub mod utils;

use std::collections::BTreeMap;

use leptos::{
    component, create_memo, create_rw_signal, spawn_local, view, For, IntoView, Show, SignalGet,
    SignalSet,
};
use leptos_meta::provide_meta_context;

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
                     <GitHubStars />

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

    }
}
