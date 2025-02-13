use std::collections::BTreeMap;

use leptos::{
    component, create_memo, create_rw_signal, spawn_local, view, For, IntoView, Show, SignalGet,
    SignalSet,
};

use proto::rustacean::Rustacean;

use crate::{components::organisms::rustacean_card::RustaceanCard, services::Services};

#[component]
pub fn Members() -> impl IntoView {
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
        <article class="safe-zone py-4">
            <h2 class="text-xl pb-2 underline">"Members"</h2>
            <p class="text-sm w-2/3">
                "People listed here form part of the Rustacean.sh Community, people from around the globe willing to share experience and participate on Rust projects."
            </p>
        </article>
        <div class="safe-zone py-4">
            <Show
                when=move || error.get().is_none()
                fallback=move || {
                    view! {
                        <span class="bg-rose-600 text-white p-4 rounded-md">
                            {error.get().unwrap_or_default()}
                        </span>
                    }
                }
            >
                <ul class="grid grid-cols-1 md:grid-cols-3 gap-4">
                    <For
                        key=|rus: &Rustacean| rus.name.clone()
                        each=move || rustaceans_list.get()
                        children=move |rustacean: Rustacean| {
                            view! { <RustaceanCard rustacean=rustacean.clone() /> }
                        }
                    />
                </ul>
            </Show>
        </div>
    }
}
