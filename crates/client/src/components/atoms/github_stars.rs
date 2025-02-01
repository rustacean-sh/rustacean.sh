use leptos::{
    component, create_render_effect, create_rw_signal, spawn_local, view, IntoView, SignalGet,
    SignalSet,
};

use crate::{components::atoms::icons::Star, services::Services};

#[component]
pub fn GitHubStars() -> impl IntoView {
    let gh_stars = create_rw_signal::<u32>(0);

    create_render_effect(move |_| {
        spawn_local(async move {
            let services = Services::new();

            match services.github().get_stars().await {
                Ok(github_option) => {
                    gh_stars.set(github_option.stargazers_count);
                    leptos::logging::log!("buscando las stars desde el atom");
                }
                Err(err) => {
                    leptos::logging::error!("{err}");
                }
            }
        });
    });

    view! {
        <a class="rsh-btn rsh-btn-secondary" href="https://github.com/rustacean-sh/rustacean.sh" target="_blank">
            <figure class="mr-2">
                <Star class="h-4 w-4 text-yellow-600" />
            </figure>
            <article class="flex">
                <span class="mr-1">
                    "Star"
                </span>
                <span>
                    {move || gh_stars.get()}
                </span>
            </article>
        </a>
    }
}
