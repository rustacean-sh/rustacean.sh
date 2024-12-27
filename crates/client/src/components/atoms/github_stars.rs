use leptos::{component, create_rw_signal, spawn_local, view, IntoView, SignalGet, SignalSet};

use crate::services::Services;
use web_sys::window;

#[component]
pub fn GitHubStars() -> impl IntoView {
    let gh_stars = create_rw_signal::<u32>(0);

    spawn_local(async move {
        let session_storage = window().unwrap().session_storage().unwrap();
        let repo_stars = session_storage.unwrap().get_item("repo_stars").unwrap();
        let services = Services::new();

        if repo_stars.is_some() {
            leptos::logging::log!(
                "There is a value in session storage from atoms {:?}",
                repo_stars.unwrap()
            );
        } else {
            match services.github().get_stars().await {
                Ok(github_stars) => {
                    gh_stars.set(github_stars.stars.unwrap());
                    leptos::logging::log!("buscando las stars desde el atom");
                }
                Err(err) => {
                    leptos::logging::error!("{err}");
                    // error.set(Some(err.to_string()));
                }
            }
        }
    });

    view! {
        <div id="github-stars" class="flex items-center gap-x-1 md:gap-x-2 ms-auto py-1 md:ps-6 md:order-3 md:col-span-3">
            <a href="https://github.com/rustacean-sh/rustacean.sh"  target="_blank" class="py-2 px-3 inline-flex items-center gap-x-2 text-sm font-medium rounded-md bg-white border border-gray-200 text-black hover:bg-gray-100 focus:outline-none focus:bg-gray-100 disabled:opacity-50 disabled:pointer-events-none ">
            <figure >
                <img  height="15" width="15" src={"../assets/images/star-svgrepo-com.svg"} alt={"Star in github"} />
            </figure>
            <span class="border-r border-gray-300 px-1">"Star"</span>
                {move || gh_stars.get()}
            </a>
        </div>
    }
}
