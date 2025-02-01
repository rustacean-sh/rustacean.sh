use leptos::{component, view, IntoView};

use proto::rustacean::Rustacean;

use crate::components::atoms::icons::GitHub;

#[component]
pub fn RustaceanCard(#[prop(into)] rustacean: Rustacean) -> impl IntoView {
    view! {
        <article class="border border-rhs-orange-400 rounded-sm overflow-hidden">
            <div class="bg-rhs-orange-400 relative h-[60px]">
                <figure class="absolute top-4 left-5 h-[70px] w-[70px] rounded-sm overflow-hidden">
                    <img height="70" width="70" src={rustacean.image} alt={rustacean.name.clone()} />
                </figure>
            </div>
            <div class="pt-10 px-4 pb-4">
                <div>
                    <strong>{rustacean.name}</strong>
                </div>
                <div class="font-mono flex flex-col items-start justify-center pt-4">
                    <a class="hover:text-rhs-orange-200 flex items-center font-medium text-sm hover:underline" target="blank" href={format!("https://github.com/{}", rustacean.gh_user)}>
                        <GitHub class="h-5 w-5" />
                        <span class="ml-1 text-xs">
                            {rustacean.gh_user}
                        </span>
                    </a>
                </div>
            </div>
        </article>
    }
}
